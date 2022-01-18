use hex_literal::hex;
use green_aureus_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
    SystemConfig, WASM_BINARY, SupplyChainConfig
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn testnet_authorities() -> Vec<(AuraId, GrandpaId)> {
    vec![(
        // 5Guv6534z7wSfZTwYSAoeXttDiKy4vXVvfQQsTGuJANbWnqY
        hex!["d67921ce411c027bd199e7ab307ce7ea1b79b3176f51d96d53a5f981475f5d0b"]
            .unchecked_into(),
        // 5DRNRGzD8CWDD1YcEmGbk5MB4rEFDGVBByDaSJW1DjV3fyG3
        hex!["3bfef4176aa107b0ba1f47fc4e2a982c1a5ab3f9a0d567bfa5967dea742aaddd"]
            .unchecked_into(),
    )]
}

pub fn testnet_superuser() -> AccountId {
    hex!["c4d162e241ccc84bc8af3353c655e9ce88d885f5b2b49edd31a1c40614c28e37"].into()
}

pub fn testnet_auditors() -> Vec<(AccountId, String)> {
    vec![
        // Finaca Gustavo
        (hex!["22556c72508eb016a83dc2c756c9b1f1a66a33762231d35f56f09953f55bae1a"].into(), String::from("Finaca Gustavo")),
        // Export Trust Limited
        (hex!["42d1d2386acf5cbbf6c349abd8dcc01bbab47c556743b2e91334f6a1fcf63712"].into(), String::from("Export Trust Limited")),
        // Safe-Transport Limited
        (hex!["fafaf10054ae89ac0820e54baec4edf9070bfa5bd3baad13b978eeab9d872332"].into(), String::from("Safe-Transport Limited")),
        // Bonaza Coffee INC.
        (hex!["dcf1afa256e61b892a78875ffc52a87f5231f031f79b106fce04baa00b271039"].into(), String::from("Bonaza Coffee INC.")), 
        // MS Phönix
        (hex!["fe6888dd220d526145cf96c33240d9e59f37ece49c135cf811d91b767544f571"].into(), String::from("Best-Shipping Limited")),
        // Spedition Pfaff
        (hex!["ba31465ce6b01b72f502cf3ab424b00fc04252fac191c2038172d06cb7c2b754"].into(), String::from("Spedition Pfaff")),   
    ]
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Testnet wasm not available".to_string())?;
    let mut endowed = vec![testnet_superuser()];
    endowed.append(&mut testnet_auditors().into_iter().map(|e| e.0).collect());

    Ok(ChainSpec::from_genesis(
        // Name
        "Green Aureus Alphanet",
        // ID
        "green_aureus_alphanet",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                testnet_authorities(),
                // Sudo account
                testnet_superuser(),
                // Pre-funded accounts
                endowed.clone(),
                false,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
        supply_chain: SupplyChainConfig {
            initial_authorities: testnet_auditors(),
        },
        transaction_payment: Default::default(),
    }
}
