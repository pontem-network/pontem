use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public, crypto::Ss58Codec};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use pontem_runtime::{
    GenesisConfig, SudoConfig, SystemConfig, BalancesConfig, WASM_BINARY, ParachainInfoConfig,
    VestingConfig, MvmConfig, TransactionPauseConfig, ParachainStakingConfig, InflationInfo,
    Range, AuthorFilterConfig, AuthorMappingConfig, TreasuryConfig, TokensConfig,
    DemocracyConfig, PolkadotXcmConfig, SchedulerConfig,
};
use primitives::{currency::CurrencyId, AccountId, Signature, Balance, BlockNumber};
use constants::SS58_PREFIX;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{include_bytes, str::from_utf8};

use nimbus_primitives::NimbusId;
use crate::vm_config::build as build_vm_config;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

// Get a public key from address.
pub fn get_public_from_address<TPublic: Public>(addr: &str) -> TPublic {
    TPublic::from_ss58check(addr).unwrap()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an account ID from address.
pub fn get_account_id_from_address(addr: &str) -> AccountId {
    AccountId::from_ss58check(addr).unwrap()
}

fn properties() -> Option<sc_chain_spec::Properties> {
    let currency = CurrencyId::default();

    json!({
        "ss58Format": SS58_PREFIX,
        "tokenDecimals": currency.decimals(),
        "tokenSymbol": from_utf8(&currency.symbol()).unwrap(),
    })
    .as_object()
    .cloned()
}

/// The list of paused extrinsics (mostly used for Nox mainnet).
fn paused_extrinsics() -> Vec<(Vec<u8>, Vec<u8>)> {
    vec![
        (
            "Balances",
            vec!["transfer", "transfer_all", "transfer_keep_alive"],
        ),
        ("Currencies", vec!["transfer", "transfer_native_currency"]),
        (
            "Vesting",
            vec!["merge_schedules", "vest", "vest_other", "vested_transfer"],
        ),
        (
            "Xtokens",
            vec![
                "transfer",
                "transfer_multiasset",
                "transfer_multiasset_with_fee",
                "transfer_with_fee",
            ],
        ),
        (
            "PolkadotXcm",
            vec![
                "execute",
                "limited_reserve_transfer_assets",
                "limited_teleport_assets",
                "reserve_transfer_assets",
                "send",
                "teleport_assets",
            ],
        ),
        ("ParachainStaking", vec!["join_candidates", "nominate"]),
        ("Treasury", vec!["propose_spend"]),
        ("Mvm", vec!["execute", "publish_module", "publish_package"]),
        (
            "MultiSig",
            vec![
                "approve_as_multi",
                "as_multi",
                "as_multi_threshold_1",
                "cancel_as_multi",
            ],
        ),
        ("Groupsign", vec!["groupsign_call"]),
        (
            "Democracy",
            vec!["propose", "note_preimage", "note_imminent_preimage"],
        ),
        (
            "AuthorMapping",
            vec!["add_association", "clear_association", "update_association"],
        ),
    ]
    .iter()
    .flat_map(|i| {
        let pallet_name = i.0.as_bytes().to_vec();
        i.1.iter()
            .map(|ex_name| (pallet_name.clone(), ex_name.as_bytes().to_vec()))
            .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
    })
    .collect()
}

/// Local development config.
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let parachain_id = ParaId::from(2000);

    Ok(ChainSpec::from_genesis(
        // Name
        "Pontem Development",
        // ID
        "pontem_dev",
        ChainType::Local,
        move || {
            genesis(
                wasm_binary,
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Candidates
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_from_seed::<NimbusId>("Alice"),
                    CurrencyId::NATIVE * 10_000,
                )],
                // Nominators
                vec![],
                // Pre-funded accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                ],
                // Vesting
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    1000,
                    150,
                    CurrencyId::NATIVE * 50_000,
                )],
                // Paused extrinsics
                vec![],
                // Parachain id
                parachain_id,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        properties(),
        // Extensions
        Extensions {
            relay_chain: "dev-service".into(),
            para_id: parachain_id.into(),
        },
    ))
}

/// Local testnet configuration.
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Testnet wasm not available".to_string())?;
    let parachain_id = ParaId::from(2000);

    Ok(ChainSpec::from_genesis(
        // Name
        "Pontem Testnet",
        // ID
        "pontem_testnet",
        ChainType::Local,
        move || {
            genesis(
                wasm_binary,
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Candidates
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_from_seed::<NimbusId>("Alice"),
                    CurrencyId::NATIVE * 10_000,
                )],
                // Nominators
                vec![],
                // Pre-funded accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Eve"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                ],
                // Vesting accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        1000,
                        150,
                        CurrencyId::NATIVE * 50_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        1000,
                        150,
                        CurrencyId::NATIVE * 50_000,
                    ),
                ],
                // Paused extrinsics
                vec![],
                // Parachain ID
                parachain_id,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("pontem_testnet"),
        // Properties
        properties(),
        // Extensions
        Extensions {
            relay_chain: "westend-local".into(),
            para_id: parachain_id.into(),
        },
    ))
}

/// Rococo configuration.
pub fn rococo_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Live wasm not available".to_string())?;
    let parachain_id = ParaId::from(2018);

    Ok(ChainSpec::from_genesis(
        // Name
        "Nox Rococo",
        // ID
        "nox_rococo",
        ChainType::Live,
        move || {
            genesis(
                wasm_binary,
                // Sudo account
                get_account_id_from_address("gkPQdcMrECsnUbVnCqTUuTaS9o72LM179rmRu3hzkC5zovUgB"),
                // Candidates
                vec![
                    // Node 1.
                    (
                        get_account_id_from_address(
                            "gkLsuHAWUiJL8tCrSYMKJjBBNyyZF2TFSs1tcTcsyHpD6x7Lr",
                        ),
                        get_public_from_address::<NimbusId>(
                            "gkLsuHAWUiJL8tCrSYMKJjBBNyyZF2TFSs1tcTcsyHpD6x7Lr",
                        ),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    // Node 2.
                    (
                        get_account_id_from_address(
                            "gkPp7Scc7zPvdPfA7YHWxsxtrzLPEW4AodGRZz9U6vqd5LFtf",
                        ),
                        get_public_from_address::<NimbusId>(
                            "gkPp7Scc7zPvdPfA7YHWxsxtrzLPEW4AodGRZz9U6vqd5LFtf",
                        ),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    // Node 3.
                    (
                        get_account_id_from_address(
                            "gkLkCGJohbgtNfXi9TkyxscHEodLvPzVUZ28MfCybvU6vN4Xn",
                        ),
                        get_public_from_address::<NimbusId>(
                            "gkLkCGJohbgtNfXi9TkyxscHEodLvPzVUZ28MfCybvU6vN4Xn",
                        ),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    // Node 4.
                    (
                        get_account_id_from_address(
                            "gkR2sZmh7tS2KgQLsByjUFHMukmGJwKgcBUshxNRAPXV5ZcZL",
                        ),
                        get_public_from_address::<NimbusId>(
                            "gkR2sZmh7tS2KgQLsByjUFHMukmGJwKgcBUshxNRAPXV5ZcZL",
                        ),
                        CurrencyId::NATIVE * 100_000,
                    ),
                ],
                // Nominators
                vec![],
                // Pre-funded accounts
                vec![
                    // Nimbus nodes.
                    (
                        // Node 1.
                        get_account_id_from_address(
                            "gkLsuHAWUiJL8tCrSYMKJjBBNyyZF2TFSs1tcTcsyHpD6x7Lr",
                        ),
                        CurrencyId::NATIVE * 110_000,
                    ),
                    (
                        // Node 2.
                        get_account_id_from_address(
                            "gkPp7Scc7zPvdPfA7YHWxsxtrzLPEW4AodGRZz9U6vqd5LFtf",
                        ),
                        CurrencyId::NATIVE * 110_000,
                    ),
                    (
                        // Node 3.
                        get_account_id_from_address(
                            "gkLkCGJohbgtNfXi9TkyxscHEodLvPzVUZ28MfCybvU6vN4Xn",
                        ),
                        CurrencyId::NATIVE * 110_000,
                    ),
                    (
                        // Node 4.
                        get_account_id_from_address(
                            "gkR2sZmh7tS2KgQLsByjUFHMukmGJwKgcBUshxNRAPXV5ZcZL",
                        ),
                        CurrencyId::NATIVE * 110_000,
                    ),
                    (
                        // Sudo.
                        get_account_id_from_address(
                            "gkPQdcMrECsnUbVnCqTUuTaS9o72LM179rmRu3hzkC5zovUgB",
                        ),
                        CurrencyId::NATIVE * 10_000,
                    ),
                    (
                        // Bank.
                        get_account_id_from_address(
                            "gkLdwjgcSFtoEvKbsgLuFBc2k6TgZxgrfj61CjcduCvgyKeux",
                        ),
                        CurrencyId::NATIVE * 200_000,
                    ),
                ],
                // Vesting accounts
                vec![],
                // Paused extrinsics
                paused_extrinsics(),
                // Parachain ID
                parachain_id,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("nox_rococo"),
        // Properties
        properties(),
        // Extensions
        Extensions {
            relay_chain: "rococo".into(),
            para_id: parachain_id.into(),
        },
    ))
}

/// NOX (Kusama) config.
/// TODO: it's still missing bootnodes, accounts, etc.
pub fn nox_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Live wasm not available".to_string())?;
    let parachain_id = ParaId::from(constants::PARACHAIN_ID);

    Ok(ChainSpec::from_genesis(
        // Name
        "Nox Mainnet",
        // ID
        "nox_mainnet",
        ChainType::Live,
        move || {
            genesis(
                wasm_binary,
                // Sudo account
                get_account_id_from_address("gkPQdcMrECsnUbVnCqTUuTaS9o72LM179rmRu3hzkC5zovUgB"),
                // Candidates
                vec![],
                // Nominators
                vec![],
                // Pre-funded accounts
                vec![],
                // Vesting accounts
                vec![],
                // Paused extrinsics
                paused_extrinsics(),
                // Parachain ID
                parachain_id,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("nox_mainnet"),
        // Properties
        properties(),
        // Extensions
        Extensions {
            relay_chain: "kusama".into(),
            para_id: parachain_id.into(),
        },
    ))
}

/// Configure initial storage state for FRAME modules.
fn genesis(
    wasm_binary: &[u8],
    root_key: AccountId,
    candidates: Vec<(AccountId, NimbusId, Balance)>,
    nominations: Vec<(AccountId, AccountId, Balance)>,
    balances: Vec<(AccountId, Balance)>,
    vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
    paused: Vec<(Vec<u8>, Vec<u8>)>,
    id: ParaId,
) -> GenesisConfig {
    let (init_module, init_func, init_args) = build_vm_config();

    let move_stdlib =
        include_bytes!("../move/move-stdlib/build/MoveStdlib/bundles/MoveStdlib.pac").to_vec();
    let pont_framework =
        include_bytes!("../move/pont-stdlib/build/PontStdlib/bundles/PontStdlib.pac").to_vec();

    GenesisConfig {
        tokens: TokensConfig { balances: vec![] },
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1000 tokens.
            balances,
        },
        parachain_system: Default::default(),
        polkadot_xcm: PolkadotXcmConfig {
            safe_xcm_version: Some(2),
        },
        parachain_info: ParachainInfoConfig { parachain_id: id },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
        parachain_staking: ParachainStakingConfig {
            candidates: candidates
                .iter()
                .cloned()
                .map(|(account, _, bond)| (account, bond))
                .collect(),
            nominations,
            inflation_config: pontem_inflation_config(),
        },
        author_filter: AuthorFilterConfig {
            eligible_ratio: sp_runtime::Percent::from_percent(50),
        },
        author_mapping: AuthorMappingConfig {
            mappings: candidates
                .iter()
                .cloned()
                .map(|(account_id, author_id, _)| (author_id, account_id))
                .collect(),
        },
        mvm: MvmConfig {
            move_stdlib,
            pont_framework,
            init_module,
            init_func,
            init_args,
            ..Default::default()
        },
        transaction_pause: TransactionPauseConfig {
            paused,
            ..Default::default()
        },
        vesting: VestingConfig { vesting },
        treasury: TreasuryConfig {},
        democracy: DemocracyConfig::default(),
        scheduler: SchedulerConfig {},
    }
}

// Pontem inflation.
pub fn pontem_inflation_config() -> InflationInfo<Balance> {
    // Let's say we have 100M total supply coins.
    InflationInfo {
        // How much staked coins we expect.
        expect: Range {
            min: CurrencyId::NATIVE * 10_000_000, // We expect to have staked at least 10M coins.
            ideal: CurrencyId::NATIVE * 20_000_000, // We expect to have staked ideal 20M coins.
            max: CurrencyId::NATIVE * 50_000_000, // We expect to have staked maximum 50M coins.
        },
        annual: Range {
            min: Perbill::from_percent(4),   // We expect minimum inflation is 4%.
            ideal: Perbill::from_percent(4), // We expect ideal inflation is 4%.
            max: Perbill::from_percent(5),   // We expect max inflation is 5%.
        },
        // 8766 rounds (hours) in a year
        round: Range {
            min: Perbill::from_parts(Perbill::from_percent(4).deconstruct() / 8766),
            ideal: Perbill::from_parts(Perbill::from_percent(4).deconstruct() / 8766),
            max: Perbill::from_parts(Perbill::from_percent(5).deconstruct() / 8766),
        },
    }
}
