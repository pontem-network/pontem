use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use pontem_runtime::{
    GenesisConfig, SudoConfig, SystemConfig, BalancesConfig, WASM_BINARY, ParachainInfoConfig,
    VestingConfig, MvmConfig, ParachainStakingConfig, InflationInfo, Range, AuthorFilterConfig,
    AuthorMappingConfig, TreasuryConfig, TokensConfig, DemocracyConfig, PolkadotXcmConfig,
    SchedulerConfig,
};
use primitives::{currency::CurrencyId, AccountId, Signature, Balance};
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

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
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

pub fn development_config(id: ParaId) -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Candidates
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_from_seed::<NimbusId>("Alice"),
                    CurrencyId::PONT * 10_000,
                )],
                // Nominators
                vec![],
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                id,
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
            relay_chain: "westend-local".into(),
            para_id: id.into(),
        },
    ))
}

pub fn local_testnet_config(id: ParaId) -> Result<ChainSpec, String> {
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
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Candidates
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_from_seed::<NimbusId>("Alice"),
                    CurrencyId::PONT * 10_000,
                )],
                // Nominators
                vec![],
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
                id,
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
            relay_chain: "westend-local".into(),
            para_id: id.into(),
        },
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    root_key: AccountId,
    candidates: Vec<(AccountId, NimbusId, Balance)>,
    nominations: Vec<(AccountId, AccountId, Balance)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
) -> GenesisConfig {
    let (init_module, init_func, init_args) = build_vm_config();

    let move_stdlib =
        include_bytes!("../move/move-stdlib/build/MoveStdlib/bundles/MoveStdlib.pac").to_vec();
    let pont_stdlib =
        include_bytes!("../move/pont-stdlib/build/PontStdlib/bundles/PontStdlib.pac").to_vec();
    let stdlib = [move_stdlib, pont_stdlib].concat();

    GenesisConfig {
        tokens: TokensConfig { balances: vec![] },
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1000 PONT.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, CurrencyId::PONT * 100_000))
                .collect(),
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
            stdlib,
            init_module,
            init_func,
            init_args,
            ..Default::default()
        },
        vesting: VestingConfig {
            // Move 10 PONT under vesting for each account since block 100 and till block 1000.
            vesting: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 100, 1000, CurrencyId::PONT * 90_000)) // K - address, 100 - when vesting starts, 1000 - how much blocks for vesting, 10 * PONT - free balance.
                .collect(),
        },
        treasury: TreasuryConfig {},
        democracy: DemocracyConfig::default(),
        scheduler: SchedulerConfig {},
    }
}

// Pontem inflation.
pub fn pontem_inflation_config() -> InflationInfo<Balance> {
    // Let's say we have 100M PONT coins.
    InflationInfo {
        // How much staked PONTs we expect.
        expect: Range {
            min: CurrencyId::PONT * 10_000_000, // We expect to have staked at least 10M PONT coins.
            ideal: CurrencyId::PONT * 20_000_000, // We expect to have staked ideal 20M PONT coins.
            max: CurrencyId::PONT * 50_000_000, // We expect to have staked maximum 50M PONT coins.
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
