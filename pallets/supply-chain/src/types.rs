extern crate alloc;

use alloc::{collections::BTreeSet, string::String, vec::Vec};
use frame_support::{parameter_types, Parameter};
use scale_info::TypeInfo;
use sp_runtime::traits::AtLeast32Bit;

pub type ComponentId = String;

/// The overarching type for a product within the supply chain.
/// Contains all the information about the parts and audits.
#[derive(
    Clone, Eq, Default, PartialEq, codec::Decode, codec::Encode, sp_runtime::RuntimeDebug, TypeInfo,
)]
pub struct Component<T>
where
    T: AtLeast32Bit + Parameter + Default + Copy,
{
    /// List of audits.
    pub audits: Vec<Audit<T>>,
    /// List of components the component is a part of.
    pub component_of: BTreeSet<ComponentId>,
    /// List of components that are part of this component.
    pub components: BTreeSet<ComponentId>,
}

/// A single audit.
#[derive(
    Clone, Eq, Default, PartialEq, codec::Decode, codec::Encode, sp_runtime::RuntimeDebug, TypeInfo,
)]
pub struct Audit<T>
where
    T: AtLeast32Bit + Parameter + Default + Copy,
{
    /// Auditor organization.
    pub auditor: String,
    /// Timestamp of the transaction.
    pub timestamp: T,
    /// Audit data in form of a JWT.
    pub audit_data: String,
}

// Define some default values for the pallet configuration.
parameter_types! {
    pub const MaxAuditorNameLength: u16 = 64;
    pub const MaxAudits: u16 = 32;
    pub const MaxAuditSize: u16 = 4096;
    pub const MaxComponents: u16 = u16::MAX;
    pub const MaxComponentIdLength: u16 = 256;
}
