/// We need mock runtime with tokens and different native currency.
/// It supports asset from Pontem (with Parachain ID == 1).
use sp_std::prelude::*;
use sp_runtime::{
    AccountId32,
    testing::Header,
    traits::{AccountIdLookup, BlakeTwo256, Convert},
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_core::RuntimeDebug;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

// Polkadot & XCM imports
use polkadot_parachain::primitives::Sibling;
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases, LocationInverter, ParentIsDefault, RelayChainAsNative,
    SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
    SovereignSignedViaLocation, EnsureXcmOrigin, AllowSubscriptionsFrom,
    AllowTopLevelPaidExecutionFrom, AllowKnownQueryResponses, TakeWeightCredit,
    FixedWeightBounds, SignedToAccountId32, AllowUnpaidExecutionFrom,
};
use xcm::latest::AssetId;
use xcm_executor::{XcmExecutor, traits::WeightTrader, Assets};
use pallet_xcm::XcmPassthrough;
use orml_traits::parameter_type_with_key;
use orml_xcm_support::{IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset};

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_balances::Call as BalancesCall;
pub use sp_runtime::{Permill, Percent, Perbill, MultiAddress};
pub use pallet_vesting::Call as VestingCall;

pub use frame_support::{
    construct_runtime, parameter_types, StorageValue, match_type,
    traits::{
        KeyOwnerProofSystem, Randomness, IsInVec, Everything, Nothing, EnsureOrigin,
        OnUnbalanced, Imbalance, Get,
    },
    weights::{
        Weight, IdentityFee, DispatchClass,
        constants::{
            BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND,
        },
    },
    PalletId,
    error::{BadOrigin},
};
use frame_system::{EnsureRoot};

/// Import the Move-pallet.
pub use sp_mvm::gas::{GasWeightMapping};
pub use sp_mvm_rpc_runtime::types::MVMApiEstimation;
pub use parachain_staking::{InflationInfo, Range};

use module_currencies::BasicCurrencyAdapter;

const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

pub type Block = frame_system::mocking::MockBlock<Runtime>;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;

pub type AccountId = AccountId32;
pub type Balance = u64;
pub type Amount = i64;
pub type Hash = sp_core::H256;
pub type BlockNumber = u64;
pub type Index = u64;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 250;
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type AccountId = AccountId;
    type Call = Call;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Event = Event;
    type Origin = Origin;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type PalletInfo = PalletInfo;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    type Version = ();
}

// Currencies id.
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
    // Relaychain's currency.
    KSM,
    // Pontem native currency.
    PONT,
    // Our native currency.
    XPONT,
}

impl CurrencyId {
    pub fn decimals(&self) -> Option<u8> {
        match self {
            Self::KSM => Some(12),
            Self::PONT => Some(10),
            Self::XPONT => Some(10),
        }
    }
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ();
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type Event = Event;
    type OnValidationData = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type OutboundXcmpMessageSource = XcmpQueue;
    type XcmpMessageHandler = XcmpQueue;
    type DmpMessageHandler = DmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type ReservedDmpWeight = ReservedDmpWeight;
}

impl parachain_info::Config for Runtime {}

parameter_types! {
    pub const RelayLocation: MultiLocation = MultiLocation::parent();
    pub const RelayNetwork: NetworkId = NetworkId::Kusama;
    pub RelayOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
    pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the default `AccountId`.
    ParentIsDefault<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<Sibling, AccountId>,
    // Straight up local `AccountId32` origins just alias directly to `AccountId`.
    AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type LocalAssetTransactor = MultiCurrencyAdapter<
    Currencies,
    (),
    IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
    AccountId,
    LocationToAccountId,
    CurrencyId,
    CurrencyIdConvert,
>;

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, Origin>,
    // Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
    // recognised.
    RelayChainAsNative<RelayOrigin, Origin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognised.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `Origin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<RelayNetwork, Origin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<Origin>,
);

parameter_types! {
     // One XCM operation is 1_000_000 weight - almost certainly a conservative estimate.
     pub UnitWeightCost: Weight = 1_000_000;
     pub const MaxInstructions: u32 = 100;
}

match_type! {
     pub type ParentOrParentsUnitPlurality: impl Contains<MultiLocation> = {
         MultiLocation { parents: 1, interior: Here } |
         MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Unit, .. }) }
     };
}

pub type Barrier = (
    TakeWeightCredit,
    AllowTopLevelPaidExecutionFrom<Everything>,
    AllowKnownQueryResponses<PolkadotXcm>,
    AllowSubscriptionsFrom<Everything>,
    AllowUnpaidExecutionFrom<ParentOrParentsUnitPlurality>,
    // ^^^ Parent & its unit plurality gets free execution
);

const PONT_PER_WEIGHT: u128 = 1_000_000;

/// Code copied from Kusama runtime. We're not using it as a dependency because of weird linkage
/// errors
mod kusama {
    pub type KusamaBalance = polkadot_primitives::v0::Balance;
    pub const UNITS: KusamaBalance = 1_000_000_000_000;
    pub const CENTS: KusamaBalance = UNITS / 30_000;
    use frame_support::weights::{
        WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
        constants::ExtrinsicBaseWeight,
    };
    use sp_runtime::Perbill;
    use smallvec::smallvec;

    pub struct KusamaWeightToFee;
    impl WeightToFeePolynomial for KusamaWeightToFee {
        type Balance = KusamaBalance;
        fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
            // in Kusama, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
            let p = CENTS;
            let q = 10 * KusamaBalance::from(ExtrinsicBaseWeight::get());
            smallvec![WeightToFeeCoefficient {
                degree: 1,
                negative: false,
                coeff_frac: Perbill::from_rational(p % q, q),
                coeff_integer: p / q,
            }]
        }
    }
}

pub struct SimpleWeightTrader(MultiLocation);
impl WeightTrader for SimpleWeightTrader {
    fn new() -> Self {
        Self(MultiLocation::parent())
    }

    fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
        let asset_id = payment
            .fungible
            .iter()
            .next()
            .expect("Payment must be something; qed")
            .0;
        let currency_id = match asset_id.clone() {
            AssetId::Concrete(multi_location) => CurrencyIdConvert::convert(multi_location),
            _ => None,
        };
        let required = match currency_id {
            Some(CurrencyId::XPONT) => asset_id
                .clone()
                .into_multiasset(Fungibility::Fungible(weight as u128 / PONT_PER_WEIGHT)),
            Some(CurrencyId::PONT) => asset_id
                .clone()
                .into_multiasset(Fungibility::Fungible(weight as u128 / PONT_PER_WEIGHT)),
            Some(CurrencyId::KSM) => {
                use frame_support::weights::WeightToFeePolynomial;
                let fee = kusama::KusamaWeightToFee::calc(&weight);
                asset_id
                    .clone()
                    .into_multiasset(Fungibility::Fungible(fee as u128))
            }
            None => asset_id
                .clone()
                .into_multiasset(Fungibility::Fungible(weight as u128)),
        };

        if let MultiAsset {
            id: Concrete(ref id),
            ..
        } = required
        {
            self.0 = id.clone();
        }
        let unused = payment
            .checked_sub(required)
            .map_err(|_| XcmError::TooExpensive)?;
        Ok(unused)
    }

    fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
        let amount = match CurrencyIdConvert::convert(self.0.clone()) {
            Some(CurrencyId::XPONT) => weight as u128 / PONT_PER_WEIGHT,
            Some(CurrencyId::PONT) => weight as u128 / PONT_PER_WEIGHT,
            Some(CurrencyId::KSM) => {
                use frame_support::weights::WeightToFeePolynomial;
                let fee = kusama::KusamaWeightToFee::calc(&weight);
                fee as u128
            }
            None => weight as u128,
        };
        Some(MultiAsset {
            id: self.0.clone().into(),
            fun: Fungibility::Fungible(amount),
        })
    }
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type Call = Call;
    type XcmSender = XcmRouter;
    // How to withdraw and deposit an asset.
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = MultiNativeAsset;
    type IsTeleporter = (); // Teleport disabled.
    type LocationInverter = LocationInverter<Ancestry>;
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
    type Trader = SimpleWeightTrader;
    type ResponseHandler = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
    type AssetTrap = PolkadotXcm;
    type AssetClaims = PolkadotXcm;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
    // Two routers - use UMP to communicate with the relay chain:
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
    // ..and XCMP to communicate with the sibling chains.
    XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
    type Event = Event;
    type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmExecuteFilter = Nothing;
    type XcmTeleportFilter = Nothing;
    type XcmReserveTransferFilter = Everything;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
    type LocationInverter = LocationInverter<Ancestry>;
    type Origin = Origin;
    type Call = Call;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
}

impl cumulus_pallet_xcm::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = PolkadotXcm;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
    fn check_inherents(
        block: &Block,
        relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
    ) -> frame_support::inherent::CheckInherentsResult {
        let relay_chain_slot = relay_state_proof
            .read_slot()
            .expect("Could not read the relay chain sloto from the proof");

        let inherent_data = cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
            relay_chain_slot,
            sp_std::time::Duration::from_secs(6),
                                                                                                                  )
            .create_inherent_data()
            .expect("Could not create the timestamp inherent data");
        inherent_data.check_extrinsics(&block)
    }
}

pub struct CurrencyIdConvert;
impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
    fn convert(id: CurrencyId) -> Option<MultiLocation> {
        match id {
            CurrencyId::KSM => Some(MultiLocation::parent()),
            CurrencyId::PONT => Some(
                (
                    Parent,
                    Junction::Parachain(1),
                    Junction::GeneralKey(b"PONT".to_vec()),
                )
                    .into(),
            ),
            CurrencyId::XPONT => Some(
                (
                    Parent,
                    Junction::Parachain(ParachainInfo::get().into()),
                    Junction::GeneralKey(b"XPONT".to_vec()),
                )
                    .into(),
            ),
        }
    }
}

impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
    fn convert(location: MultiLocation) -> Option<CurrencyId> {
        match location {
            MultiLocation {
                parents: 1,
                interior: Junctions::Here,
            } => Some(CurrencyId::KSM),
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } => {
                if id == 1 && key == b"PONT".to_vec() {
                    return Some(CurrencyId::PONT);
                }

                if key == b"XPONT".to_vec() {
                    return Some(CurrencyId::XPONT);
                }

                None
            }
            _ => None,
        }
    }
}

impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
    fn convert(asset: MultiAsset) -> Option<CurrencyId> {
        if let MultiAsset {
            id: Concrete(id),
            fun: _,
        } = asset
        {
            Self::convert(id)
        } else {
            None
        }
    }
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        100
    };
}

impl orml_tokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = MaxLocks;
    type DustRemovalWhitelist = Everything;
}

parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = CurrencyId::XPONT;
}

impl module_currencies::Config for Runtime {
    type Event = Event;
    type CurrencyId = CurrencyId;
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type WeightInfo = ();
    type SweepOrigin = EnsureRoot<AccountId>;
    type OnDust = ();
}

pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
    fn convert(account: AccountId) -> MultiLocation {
        X1(Junction::AccountId32 {
            network: NetworkId::Any,
            id: account.into(),
        })
        .into()
    }
}

parameter_types! {
    pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::get().into())));
    pub const BaseXcmWeight: Weight = 100_000_000;
}

impl orml_xtokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type CurrencyIdConvert = CurrencyIdConvert;
    type AccountIdToMultiLocation = AccountIdToMultiLocation;
    type SelfLocation = SelfLocation;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
    type BaseXcmWeight = BaseXcmWeight;
    type LocationInverter = LocationInverter<Ancestry>;
}

impl orml_xcm::Config for Runtime {
    type Event = Event;
    type SovereignOrigin = EnsureRoot<AccountId>;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Config, Storage, Inherent, Event<T>} = 20,
        ParachainInfo: parachain_info::{Pallet, Storage, Config} = 21,

        // Balances.
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 30,
        Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
        Currencies: module_currencies::{Pallet, Call, Storage, Event<T>},

        // XCM helpers
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 60,
        PolkadotXcm: pallet_xcm::{Pallet, Config, Call, Event<T>, Storage, Origin} = 61,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Call, Event<T>, Origin} = 62,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 63,
        Xtokens: orml_xtokens::{Pallet, Storage, Call, Event<T>} = 64,
        OrmlXcm: orml_xcm::{Pallet, Call, Event<T>} = 65,
    }
);

cumulus_pallet_parachain_system::register_validate_block!(
    Runtime = Runtime,
    BlockExecutor = pallet_author_inherent::BlockExecutor::<Runtime, Executive>,
    CheckInherents = CheckInherents,
);
