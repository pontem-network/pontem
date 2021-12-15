pub use crate::*;
use frame_support::sp_io::TestExternalities;
use frame_support::traits::GenesisBuild;
use sp_core::crypto::Ss58Codec;
use frame_support::traits::Hooks;
use std::include_bytes;
use move_vm::genesis::GenesisConfig;

// Genesis configuration for Move VM.
pub type ModuleName = Vec<u8>;
pub type FunctionName = Vec<u8>;
pub type FunctionArgs = Vec<Vec<u8>>;
pub fn build_vm_config() -> (ModuleName, FunctionName, FunctionArgs) {
    // We use standard arguments.
    let genesis: GenesisConfig = Default::default();

    (
        b"Genesis".to_vec(),
        b"initialize".to_vec(),
        genesis.init_func_config.unwrap().args,
    )
}

/// User accounts.
pub enum Accounts {
    ALICE,
    BOB,
}

impl Accounts {
    /// Convert account to AccountId.
    pub fn account(&self) -> AccountId {
        match self {
            Accounts::ALICE => {
                AccountId::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
                    .unwrap()
            }
            Accounts::BOB => {
                AccountId::from_ss58check("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")
                    .unwrap()
            }
        }
    }
}

impl Into<[u8; 32]> for Accounts {
    fn into(self) -> [u8; 32] {
        self.account().into()
    }
}

/// Balance to currency unit (e.g. 1 PONT).
pub fn to_unit(amount: Balance, currency_id: CurrencyId) -> Balance {
    amount * u64::pow(10, currency_id.decimals() as u32)
}

/// Roll till next block.
pub fn run_to_block(till: u32) {
    while System::block_number() < till {
        Mvm::on_finalize(System::block_number());
        Scheduler::on_finalize(System::block_number());
        Balances::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        Timestamp::set_timestamp(System::block_number() as u64 * 12000);
        Scheduler::on_initialize(System::block_number());
        ParachainStaking::on_initialize(System::block_number());
    }
}

/// Runtime builder.
pub struct RuntimeBuilder {
    balances: Vec<(AccountId, CurrencyId, Balance)>,
    vesting: Vec<(AccountId, BlockNumber, u32, Balance)>,
    parachain_id: Option<u32>,
}

impl RuntimeBuilder {
    /// Create new Runtime builder instance.
    pub fn new() -> Self {
        Self {
            balances: vec![],
            vesting: vec![],
            parachain_id: None,
        }
    }

    /// Set balances.
    pub fn set_balances(mut self, balances: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    /// Set vesting.
    pub fn set_vesting(mut self, vesting: Vec<(AccountId, BlockNumber, u32, Balance)>) -> Self {
        self.vesting = vesting;
        self
    }

    /// Parachain id.
    pub fn set_parachain_id(mut self, parachain_id: u32) -> Self {
        self.parachain_id = Some(parachain_id);
        self
    }

    /// Build Runtime for testing.
    pub fn build(self) -> TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        let native_currency_id = GetNativeCurrencyId::get();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self
                .balances
                .clone()
                .into_iter()
                .filter(|(_, currency_id, _)| *currency_id == native_currency_id)
                .map(|(account_id, _, initial_balance)| (account_id, initial_balance))
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_vesting::GenesisConfig::<Runtime> {
            vesting: self.vesting,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        orml_tokens::GenesisConfig::<Runtime> {
            balances: self
                .balances
                .into_iter()
                .filter(|(_, currency_id, _)| *currency_id != native_currency_id)
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        <parachain_info::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &parachain_info::GenesisConfig {
                parachain_id: self.parachain_id.unwrap_or(constants::PARACHAIN_ID).into(),
            },
            &mut t,
        )
        .unwrap();

        <pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &pallet_xcm::GenesisConfig {
                safe_xcm_version: Some(2),
            },
            &mut t,
        )
        .unwrap();

        let (init_module, init_func, init_args) = build_vm_config();
        sp_mvm::GenesisConfig::<Runtime> {
            stdlib: include_bytes!("./assets/stdlib/artifacts/bundles/move-stdlib.pac").to_vec(),
            init_module,
            init_func,
            init_args,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
