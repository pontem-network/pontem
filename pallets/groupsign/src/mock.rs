use crate::{self as groupsign, weights::PontemWeights};
use codec::{Decode, Encode};
use frame_support::{parameter_types, traits::ConstU32};
use frame_system as system;
use scale_info::TypeInfo;
use sp_core::{H256, sr25519};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup, Lazy, Verify},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = sp_core::sr25519::Public;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Groupsign: groupsign::{Pallet, Call, Origin<T>, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<12>;
}

impl groupsign::Config for Test {
    type Event = Event;

    type WeightInfo = PontemWeights<Self>;
    type MyOrigin = Origin;

    type Call = Call;
    type Public = AccountId;
    type Signature = AnySignature;
}

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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
