//! This module offers supply chain functionality. It allows the superuser to authorize
//! organizations to add audits and new components. Authorized organizations can add
//! audits to existing components or add new components.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
    extern crate alloc;

    use super::types::{Audit, Component, ComponentId};
    use alloc::{string::String, vec::Vec};
    use frame_support::{pallet_prelude::*, traits::Time, transactional};
    use frame_system::pallet_prelude::*;
    use sp_runtime::SaturatedConversion;

    type MomentOf<T> = <<T as Config>::Timestamp as Time>::Moment;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The maximum length of the auditor name.
        #[pallet::constant]
        type MaxAuditorNameLength: Get<u16>;

        /// The maximum number of audits a product can have.
        #[pallet::constant]
        type MaxAudits: Get<u16>;

        /// The maximum size (in bytes) the audit data can have.
        #[pallet::constant]
        type MaxAuditSize: Get<u16>;

        /// The maximum length of the component id.
        #[pallet::constant]
        type MaxComponentIdLength: Get<u16>;

        /// The maximum number of parts a product can have and be part of.
        #[pallet::constant]
        type MaxComponents: Get<u16>;

        /// Pallet that is used to retrieve timestamps.
        type Timestamp: Time;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Maps a ComponentId to Component structure, which contains other parts audits.
    #[pallet::storage]
    #[pallet::getter(fn components)]
    pub(super) type Components<T> =
        StorageMap<_, Blake2_128Concat, ComponentId, Component<MomentOf<T>>, ValueQuery>;

    /// Maps an organization account to an organization name. Will be used when an audit is added.
    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub(super) type Authorities<T> =
        StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, String, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An account was mapped to an organization and granted permission to add audits.
        /// \[account, organization\]
        AuthorityAdded(T::AccountId, String),
        /// An account was removed from the set of authorized accounts.
        /// \[account, organization\]
        AuthorityRemoved(T::AccountId, String),
        /// A ComponentId was seen for the first time and a Component entry was created. \[part_id\]
        ComponentCreated(ComponentId),
        /// An audit was added to a part \[part_id, organization\]
        AuditAdded(ComponentId, String),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Tried to authorize an account that is already authorized.
        AlreadyAuthorized,
        /// The name of the auditing organization is too long.
        AuditorNameTooLong,
        /// Audit size is too big.
        AuditTooBig,
        /// Tried to remove authorization from an account that is not authorized.
        AuthorityNotFound,
        /// It was tried to create a component that already exists.
        ComponentAlreadyExists,
        /// The component's id is too long.
        ComponentIdTooLong,
        /// Some data that was provided was empty.
        EmptyDataProvided,
        /// Maximum number of audits reached.
        MaxAuditsReached,
        /// A component has reached the maximum number of components.
        MaxComponentsReached,
        /// A component has reached the maximum number of components it is part of.
        MaxComponentOfReached,
        /// No permission to add audits.
        Unauthorized,
    }

    #[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_authorities: Vec<(T::AccountId, String)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { initial_authorities: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
            for (acc, name) in &self.initial_authorities {
                <Authorities<T>>::insert(acc, name);
            }
		}
	}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// The superuser can authorize organizations to submit audits.
        ///
        /// # Parameters
        ///
        /// * `account`: Account id to authorize.
        /// * `name`: Name associated with that account_id.
        #[pallet::weight(1_000_000 + T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1))]
        pub fn authorize(
            origin: OriginFor<T>,
            account: T::AccountId,
            name: String,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                name.len() > 0 && name != String::default(),
                <Error<T>>::EmptyDataProvided
            );
            ensure!(
                name.len() <= T::MaxAuditorNameLength::get().saturated_into(),
                <Error<T>>::AuditorNameTooLong
            );

            <Authorities<T>>::try_mutate(account.clone(), |val| -> DispatchResult {
                ensure!(*val == String::default(), <Error<T>>::AlreadyAuthorized);

                *val = name.clone();
                Ok(())
            })?;

            Self::deposit_event(<Event<T>>::AuthorityAdded(account, name));
            Ok(())
        }

        /// The superuser can de-authorize organizations, such that they can't submit audits.
        ///
        /// # Parameters
        ///
        /// * `account`: Account id to de-authorize.
        #[pallet::weight(1_000_000 + T::DbWeight::get().reads(1))]
        pub fn deauthorize(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            let name = <Authorities<T>>::take(account.clone());
            ensure!(name != String::default(), <Error<T>>::AuthorityNotFound);
            Self::deposit_event(<Event<T>>::AuthorityRemoved(account, name));
            Ok(())
        }

        /// Add an audit to a component.
        ///
        /// # Parameters
        ///
        /// * `component_id`: Id of the component to add an audit for.
        /// * `audit_data`: JWT data containing the audit.
        #[pallet::weight(1_000_000 + T::DbWeight::get().reads(2) + T::DbWeight::get().writes(1))]
        pub fn audit(
            origin: OriginFor<T>,
            audit_data: String,
            component_id: ComponentId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::checked_add_audit(who, audit_data, None, component_id)
        }

        /// Add an audit to a component. This is called when a new component is crafted.
        /// Any components that are integrated into the new component must be listed.
        ///
        /// # Parameters
        ///
        /// * `component_id`: Id of the component to add an audit for.
        /// * `components`: A list of components that are part of the new component.
        /// * `audit_data`: JWT data containing the audit.
        #[pallet::weight(1_000_000 + T::DbWeight::get().reads(2) + T::DbWeight::get().writes(1))]
        #[transactional]
        pub fn audit_assembly(
            origin: OriginFor<T>,
            audit_data: String,
            components: Vec<ComponentId>,
            component_id: ComponentId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Ensure thisd component does not exist, otherwise: If it exists, why is it created now?
            Self::checked_add_audit(who, audit_data, Some(components), component_id.clone())?;
            Ok(())
        }
    }

    /// Add an audit to a component.
    ///
    /// # Parameters
    ///
    /// * `auditor`: AccountId of the auditor.
    /// * `component_id`: Id of the component to add an audit for.
    /// * `audit_data`: JWT data containing the audit.
    impl<T: Config> Pallet<T> {
        pub(super) fn checked_add_audit(
            auditor: T::AccountId,
            audit_data: String,
            components: Option<Vec<ComponentId>>,
            component_id: ComponentId,
        ) -> DispatchResult {
            ensure!(
                component_id.len() <= T::MaxComponentIdLength::get().saturated_into(),
                <Error<T>>::ComponentIdTooLong
            );
            ensure!(audit_data.len() != 0, <Error<T>>::EmptyDataProvided);
            ensure!(
                audit_data.len() <= T::MaxAuditSize::get().saturated_into(),
                <Error<T>>::AuditTooBig
            );
            let name = Self::authorities(auditor);
            ensure!(name != String::default(), <Error<T>>::Unauthorized);

            <Components<T>>::try_mutate(component_id.clone(), |val| -> DispatchResult {
                ensure!(
                    val.audits.len() < T::MaxAudits::get().saturated_into(),
                    <Error<T>>::MaxAuditsReached
                );

                let emit_component_added = *val == Default::default();

                if let Some(comps) = components {
                    // We should only create a component if it does not already exist.
                    ensure!(
                        val.audits.len() == 0
                            && val.components.len() == 0
                            && val.component_of.len() == 0,
                        <Error<T>>::ComponentAlreadyExists
                    );
                    ensure!(
                        comps.len() <= T::MaxComponents::get().saturated_into(),
                        <Error<T>>::MaxComponentsReached
                    );

                    for component in comps.iter() {
                        <Components<T>>::try_mutate(component, |other| -> DispatchResult {
                            ensure!(
                                other.component_of.len() < T::MaxComponents::get().saturated_into(),
                                <Error<T>>::MaxComponentOfReached
                            );
                            other.component_of.insert(component_id.clone());
                            Ok(())
                        })?;
                    }

                    val.components = comps.into_iter().collect();
                }

                if emit_component_added {
                    Self::deposit_event(<Event<T>>::ComponentCreated(component_id.clone()));
                }

                val.audits.push(Audit {
                    auditor: name.clone(),
                    timestamp: T::Timestamp::now(),
                    audit_data,
                });

                Self::deposit_event(<Event<T>>::AuditAdded(component_id, name));
                Ok(())
            })
        }
    }
}
