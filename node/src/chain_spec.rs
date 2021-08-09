use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sp_runtime::{traits::{IdentifyAccount, Verify}, Perbill};
use mv_node_runtime::{
    GenesisConfig, SudoConfig, SystemConfig, BalancesConfig, WASM_BINARY, AccountId, Signature, ParachainStakingConfig,
    ParachainInfoConfig, VestingConfig, PONT, DECIMALS, InflationInfo, Range, Balance, AuthorFilterConfig, AuthorMappingConfig,
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use nimbus_primitives::NimbusId;

/// Address format for Pontem.
/// 42 is a placeholder for any Substrate-based chain.
/// See https://github.com/paritytech/substrate/blob/master/ss58-registry.json
const SS58_FORMAT: u8 = 42;

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
    json!({
        "ss58Format": SS58_FORMAT,
        "tokenDecimals": DECIMALS,
        "tokenSymbol": "PONT"
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
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_from_seed::<NimbusId>("Alice"),
                        10_000 * PONT,
                    )
                ],
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
            relay_chain: "rococo-local".into(),
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
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_from_seed::<NimbusId>("Alice"),
                        10_000 * PONT,
                    )
                ],
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
            relay_chain: "rococo-local".into(),
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
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1000 PONT.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 100_000 * PONT))
                .collect(),
        },
        parachain_system: Default::default(),
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
        vesting: VestingConfig {
            // Move 10 PONT under vesting for each account since block 100 and till block 1000.
            vesting: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 100, 1000, 90_000 * PONT)) // K - address, 100 - when vesting starts, 1000 - how much blocks for vesting, 10 * PONT - free balance.
                .collect(),
        },
    }
}

// Pontem inflation.
pub fn pontem_inflation_config() -> InflationInfo<Balance> {
	InflationInfo {
		expect: Range {
			min: 100_000 * PONT,
			ideal: 200_000 * PONT,
			max: 500_000 * PONT,
		},
		annual: Range {
			min: Perbill::from_percent(4),
			ideal: Perbill::from_percent(5),
			max: Perbill::from_percent(5),
		},
		// 8766 rounds (hours) in a year
		round: Range {
			min: Perbill::from_parts(Perbill::from_percent(4).deconstruct() / 8766),
			ideal: Perbill::from_parts(Perbill::from_percent(5).deconstruct() / 8766),
			max: Perbill::from_parts(Perbill::from_percent(5).deconstruct() / 8766),
		},
	}
}
