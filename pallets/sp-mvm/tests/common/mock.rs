#![allow(dead_code)]
/// Mock runtime.
use groupsign::weights::PontemWeights;
use sp_mvm::gas;
use sp_core::{H256, sr25519};
use sp_std::{convert::TryFrom, fmt::Debug};
use frame_system as system;
use parity_scale_codec::{Decode, Encode};
use system::EnsureRoot;
use frame_support::{
    PalletId, parameter_types,
    traits::{Everything, ConstU32},
    weights::{Weight, constants::WEIGHT_PER_SECOND},
};
use sp_std::vec;
use std::include_bytes;
use frame_support::traits::{OnInitialize, OnFinalize};
use sp_runtime::traits::{Verify, Lazy, BlakeTwo256, IdentityLookup, ConvertInto};
use sp_runtime::{testing::Header};
use orml_traits::parameter_type_with_key;
use constants::SS58_PREFIX;
use scale_info::TypeInfo;

pub use primitives::currency::CurrencyId;
use module_currencies::BasicCurrencyAdapter;

use super::vm_config::build as build_vm_config;

type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Test>;
type Block = system::mocking::MockBlock<Test>;

/// Initial balance for all existent test accounts
pub const INITIAL_BALANCE: <Test as balances::Config>::Balance = 42000;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

// Implement signature just for test.
#[derive(Eq, PartialEq, Clone, Encode, Decode, TypeInfo, Debug)]
pub struct AnySignature(sr25519::Signature);

impl Verify for AnySignature {
    type Signer = sr25519::Public;
    fn verify<L: Lazy<[u8]>>(&self, mut msg: L, signer: &sr25519::Public) -> bool {
        let msg = msg.get();
        self.0.verify(msg, signer)
    }
}

impl From<sr25519::Signature> for AnySignature {
    fn from(s: sr25519::Signature) -> Self {
        Self(s)
    }
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: timestamp::{Pallet, Call, Storage, Inherent},
        Balances: balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Vesting: pallet_vesting::{Pallet, Call, Storage, Config<T>, Event<T>},
        Tokens: orml_tokens::{Pallet, Storage, Event<T>},
        Currencies: module_currencies::{Pallet, Call, Storage, Event<T>},
        Mvm: sp_mvm::{Pallet, Call, Config<T>, Storage, Event<T>},
        Groupsign: groupsign::{Pallet, Call, Origin<T>, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = SS58_PREFIX;
}

pub type AccountId = sp_core::sr25519::Public;
pub type Amount = i64;
pub type BlockNumber = u64;
pub type Balance = u64;

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    // type AccountData = ();
    type AccountData = balances::AccountData<<Self as balances::Config>::Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<12>;
}

// --- gas --- //
/// By inheritance from Moonbeam and from Dfinance (based on validators statistic), we believe max 4125000 gas is currently enough for block.
/// In the same time we use same 500ms Weight as Max Block Weight, from which 75% only are used for transactions.
/// So our max gas is GAS_PER_SECOND * 0.500 * 0.65 => 4125000.
pub const GAS_PER_SECOND: u64 = 11_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND / GAS_PER_SECOND;

pub struct MoveVMGasWeightMapping;

// Just use provided gas.
impl gas::GasWeightMapping for MoveVMGasWeightMapping {
    fn gas_to_weight(gas: u64) -> Weight {
        gas.saturating_mul(WEIGHT_PER_GAS)
    }

    fn weight_to_gas(weight: Weight) -> u64 {
        u64::try_from(weight.wrapping_div(WEIGHT_PER_GAS)).unwrap_or(u32::MAX as u64)
    }
}
// ----------------- //

// --- timestamp --- //

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}
impl timestamp::Config for Test {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

// --- balances --- //

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
    pub const TransferFee: u64 = 1 * MILLIUNIT;
    pub const CreationFee: u64 = 1 * MILLIUNIT;
    pub const TransactionByteFee: u64 = 1 * MILLIUNIT;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl balances::Config for Test {
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = Sys;
    type WeightInfo = balances::weights::SubstrateWeight<Self>;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const MinVestedTransfer: Balance = 1;
}

impl pallet_vesting::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = ();
    const MAX_VESTING_SCHEDULES: u32 = 1;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        100
    };
}

impl orml_tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = primitives::Amount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = MaxLocks;
    type DustRemovalWhitelist = Everything;
}

parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = CurrencyId::NATIVE;
}
impl module_currencies::Config for Test {
    type Event = Event;
    type CurrencyId = CurrencyId;
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type WeightInfo = ();
    type SweepOrigin = EnsureRoot<AccountId>;
    type OnDust = ();
}

// -------- move vm pallet --------- //
parameter_types! {
    pub const MVMPalletId: PalletId = PalletId(*b"_nox/mvm");
}
impl sp_mvm::Config for Test {
    type Event = Event;
    type GasWeightMapping = MoveVMGasWeightMapping;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type PalletId = MVMPalletId;
    type CurrencyId = CurrencyId;
    type Currencies = Currencies;
    type WeightInfo = ();
}

parameter_types! {
    pub const DepositBase: u64 = 0;
    pub const DepositFactor: u64 = 0;
    pub const MaxSignatories: u16 = 16;
}

impl groupsign::Config for Test {
    type Event = Event;
    type Call = Call;
    type Public = AccountId;
    type Signature = AnySignature;
    type MyOrigin = Origin;
    type WeightInfo = PontemWeights<Self>;
}

pub type Sys = system::Pallet<Test>;
pub type Time = timestamp::Pallet<Test>;
pub type MoveEvent = sp_mvm::Event<Test>;

/// Runtime builder.
pub struct RuntimeBuilder {
    balances: Vec<(AccountId, CurrencyId, Balance)>,
    vesting: Vec<(AccountId, BlockNumber, u32, Balance)>,
}

impl RuntimeBuilder {
    /// Create new Runtime builder instance.
    pub fn new() -> Self {
        Self {
            balances: vec![],
            vesting: vec![],
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

    /// Build genesis storage according to the mock runtime.
    pub fn build(self) -> sp_io::TestExternalities {
        let mut sys = system::GenesisConfig::default()
            .build_storage::<Test>()
            .expect("Frame system builds valid default genesis config");

        let native_currency_id = GetNativeCurrencyId::get();

        balances::GenesisConfig::<Test> {
            balances: self
                .balances
                .clone()
                .into_iter()
                .filter(|(_, currency_id, _)| *currency_id == native_currency_id)
                .map(|(account_id, _, initial_balance)| (account_id, initial_balance))
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut sys)
        .expect("Pallet balances storage can't be assimilated");

        let vm_config = build_vm_config();

        let move_stdlib =
            include_bytes!("../assets/move-stdlib/build/MoveStdlib/bundles/MoveStdlib.pac")
                .to_vec();
        let pont_framework =
            include_bytes!("../assets/pont-stdlib/build/PontStdlib/bundles/PontStdlib.pac")
                .to_vec();

        sp_mvm::GenesisConfig::<Test> {
            move_stdlib,
            pont_framework,
            init_module: vm_config.0.clone(),
            init_func: vm_config.1.clone(),
            init_args: vm_config.2.clone(),
            ..Default::default()
        }
        .assimilate_storage(&mut sys)
        .expect("Pallet mvm storage can't be assimilated");

        sys.into()
    }
}

/// Timestamp multiplier.
pub const TIME_BLOCK_MULTIPLIER: u64 = 100;

/// Roll next block.
pub fn roll_next_block() {
    Balances::on_finalize(Sys::block_number());
    Mvm::on_finalize(Sys::block_number());
    Sys::on_finalize(Sys::block_number());
    Sys::set_block_number(Sys::block_number() + 1);
    Sys::on_initialize(Sys::block_number());
    Mvm::on_initialize(Sys::block_number());
    Balances::on_initialize(Sys::block_number());

    // set time with multiplier `*MULTIPLIER` by block:
    Time::set_timestamp(Sys::block_number() * TIME_BLOCK_MULTIPLIER);

    println!("now block: {}, time: {}", Sys::block_number(), Time::get());
}

/// Roll to block N.
pub fn roll_block_to(n: u64) {
    while Sys::block_number() < n {
        roll_next_block()
    }
}

/// Get last event.
pub fn last_event() -> Event {
    {
        let events = Sys::events();
        println!("events: {:?}", events);
    }
    Sys::events().pop().expect("Event expected").event
}

/// If no events recently.
pub fn have_no_events() -> bool {
    Sys::events().is_empty()
}
