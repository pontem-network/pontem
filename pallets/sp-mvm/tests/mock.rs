#![allow(dead_code)]

use sp_mvm::{Module, Trait, gas};
use sp_core::H256;
use sp_std::convert::TryFrom;
use frame_system as system;
use frame_support::{
    impl_outer_origin, impl_outer_event, parameter_types,
    weights::{Weight, constants::WEIGHT_PER_SECOND},
};
use frame_support::traits::{OnInitialize, OnFinalize};
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use sp_runtime::{testing::Header, Perbill};

impl_outer_origin! {
    pub enum Origin for Test {}
    // pub enum Origin for Test where system = frame_system {}
}

impl_outer_event! {
    pub enum TestEvent for Test {
        sp_mvm<T>,
        system<T>,
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sp_core::sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = TestEvent;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

pub const GAS_PER_SECOND: u64 = 8_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND / GAS_PER_SECOND;

pub struct MoveVMGasWeightMapping;

// Just use provided gas.
impl gas::GasWeightMapping for MoveVMGasWeightMapping {
    fn gas_to_weight(gas: u64) -> Weight {
        Weight::try_from((gas).saturating_mul(WEIGHT_PER_GAS)).unwrap_or(Weight::MAX)
    }

    fn weight_to_gas(weight: Weight) -> u64 {
        u64::try_from(weight.wrapping_div(WEIGHT_PER_GAS)).unwrap_or(u64::MAX)
    }
}

impl Trait for Test {
    type Event = TestEvent;
    type GasWeightMapping = MoveVMGasWeightMapping;
}

pub type Mvm = Module<Test>;
pub type Sys = system::Module<Test>;
pub type MoveEvent = sp_mvm::Event<Test>;

/// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn roll_next_block() {
    // Stake::on_finalize(Sys::block_number());
    // Balances::on_finalize(Sys::block_number());
    Mvm::on_finalize(Sys::block_number());
    Sys::on_finalize(Sys::block_number());
    Sys::set_block_number(Sys::block_number() + 1);
    Sys::on_initialize(Sys::block_number());
    Mvm::on_initialize(Sys::block_number());
    // Balances::on_initialize(Sys::block_number());
    // Stake::on_initialize(Sys::block_number());
    println!("current block number: {}", Sys::block_number());
}

pub fn roll_block_to(n: u64) {
    while Sys::block_number() < n {
        roll_next_block()
    }
}

pub fn last_event() -> TestEvent {
    {
        let events = Sys::events();
        println!("events: {:?}", events);
    }
    Sys::events().pop().expect("Event expected").event
}

pub fn have_no_events() -> bool {
    Sys::events().is_empty()
}
