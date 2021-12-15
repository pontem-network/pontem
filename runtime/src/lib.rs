#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_std::prelude::*;
use sp_core::OpaqueMetadata;
use sp_runtime::{
    ApplyExtrinsicResult, create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, Convert, ConvertInto},
    transaction_validity::{TransactionValidity, TransactionSource},
};
use sp_api::impl_runtime_apis;
use sp_version::RuntimeVersion;
#[cfg(feature = "std")]
use sp_version::NativeVersion;

use cumulus_pallet_parachain_system::RelaychainBlockNumberProvider;
use nimbus_primitives::NimbusId;

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
use pallet_balances::NegativeImbalance;
pub use sp_runtime::{Permill, Percent, Perbill, MultiAddress};
pub use pallet_vesting::Call as VestingCall;

pub use frame_support::{
    pallet_prelude::RuntimeDebug,
    construct_runtime, parameter_types, StorageValue, match_type,
    traits::{
        KeyOwnerProofSystem, Randomness, IsInVec, Everything, Nothing, EnsureOrigin,
        OnUnbalanced, Imbalance, Get, EqualPrivilegeOnly,
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
use frame_system::{
    EnsureRoot, RawOrigin,
    limits::{BlockLength, BlockWeights},
};

/// Import the Move-pallet.
pub use sp_mvm::gas::{GasWeightMapping};
pub use sp_mvm_rpc_runtime::types::MVMApiEstimation;
pub use parachain_staking::{InflationInfo, Range};

use constants::{SS58_PREFIX, currency::*, time::*};
use primitives::{*, currency::CurrencyId, Index};

use module_currencies::BasicCurrencyAdapter;

#[cfg(test)]
mod tests;

/// We allow for 0.5 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

impl_opaque_keys! {
    pub struct SessionKeys {
        pub nimbus: AuthorInherent,
    }
}

// To learn more about runtime versioning and what each of the following value means:
//   https://substrate.dev/docs/en/knowledgebase/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("pontem-node"),
    impl_name: create_runtime_str!("pontem-node"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
    //   the compatible custom types.
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

/// 1 in 4 blocks (on average) will be primary babe blocks
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// We assume that ~20% of the block weight is consumed by `on_initalize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// We allow `Normal` extrinsics to fill up the block up to 65%, the rest can be used
/// by  Operational  extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(65);

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
    pub const BlockHashCount: BlockNumber = 2400;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub RuntimeBlockLength: BlockLength = BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = SS58_PREFIX;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain. 105 is the Pontem Network.
    type SS58Prefix = SS58Prefix;
    /// What to do if the user wants the code set to something. Just use `()` unless you are in
    /// cumulus.
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
}

parameter_types! {
    pub const EnactmentPeriod: BlockNumber = 3 * DAYS;
    pub const LaunchPeriod: BlockNumber = 3 * DAYS;
    pub const VotingPeriod: BlockNumber = 5 * DAYS;
    pub const VoteLockingPeriod: BlockNumber = 1 * DAYS;
    pub const FastTrackVotingPeriod: BlockNumber = 3 * HOURS;
    pub const CooloffPeriod: BlockNumber = 14 * DAYS;

    // 100 PONT as minimum deposit.
    pub const MinimumDeposit: Balance = CurrencyId::PONT.times(100);

    // e.g. 100 PONT for 1 MB.
    pub const PreimageByteDeposit: Balance = 1000000;

    pub const MaxVotes: u32 = 100;
    pub const MaxProposals: u32 = 100;

    // Allow emergency.
    pub const InstantAllowed: bool = true;
}
pub struct AssumeRootIsSudo();
impl EnsureOrigin<Origin> for AssumeRootIsSudo {
    type Success = AccountId;
    fn try_origin(o: Origin) -> Result<Self::Success, Origin> {
        let f: Result<_, _> = o.into();
        f.and_then(|t| match t {
            RawOrigin::Root => Ok(Sudo::key()),
            r => Err(Origin::from(r)),
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> Origin {
        Origin::from(RawOrigin::Root)
    }
}

impl pallet_democracy::Config for Runtime {
    type Proposal = Call;
    type Event = Event;
    type Currency = Balances;

    type EnactmentPeriod = EnactmentPeriod;
    type LaunchPeriod = LaunchPeriod;
    type VotingPeriod = VotingPeriod;
    type VoteLockingPeriod = VoteLockingPeriod;
    type FastTrackVotingPeriod = FastTrackVotingPeriod;

    type MinimumDeposit = MinimumDeposit;

    type ExternalOrigin = EnsureRoot<AccountId>;
    type ExternalMajorityOrigin = EnsureRoot<AccountId>;
    type ExternalDefaultOrigin = EnsureRoot<AccountId>;
    type FastTrackOrigin = EnsureRoot<AccountId>;
    type CancellationOrigin = EnsureRoot<AccountId>;
    type BlacklistOrigin = EnsureRoot<AccountId>;
    type CancelProposalOrigin = EnsureRoot<AccountId>;
    type VetoOrigin = AssumeRootIsSudo;
    type OperationalPreimageOrigin = AssumeRootIsSudo;

    type CooloffPeriod = CooloffPeriod;
    type PreimageByteDeposit = PreimageByteDeposit;
    type Slash = ();
    type InstantOrigin = EnsureRoot<AccountId>;
    type InstantAllowed = InstantAllowed;
    type Scheduler = Scheduler;
    type MaxVotes = MaxVotes;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_democracy::weights::SubstrateWeight<Runtime>;
    type MaxProposals = MaxProposals;
}

parameter_types! {
    pub const SpendPeriod: BlockNumber = 6 * DAYS;
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    /// 5% of proposal value should be reserved from proposer balance,
    /// but not less than ProposalBondMinimum.
    /// This value would be slashed if proposal rejected.
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = CurrencyId::PONT.times(100);
    pub const MaxApprovals: u32 = 100;
}

impl pallet_treasury::Config for Runtime {
    type Currency = Balances;
    // Only root for now, governance later.
    type ApproveOrigin = EnsureRoot<AccountId>;
    // Only root for now, governance later.
    type RejectOrigin = EnsureRoot<AccountId>;
    type PalletId = TreasuryPalletId;
    type MaxApprovals = MaxApprovals;
    type Event = Event;
    // If proposal rejected - send deposit to treasury.
    type OnSlash = Treasury;
    type ProposalBond = ProposalBond;
    type ProposalBondMinimum = ProposalBondMinimum;
    type SpendPeriod = SpendPeriod;
    // Not burning.
    type Burn = ();
    type BurnDestination = ();
    type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
    type SpendFunds = ();
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * RuntimeBlockWeights::get().max_block;
    pub const MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: Balance = PONT_EXISTENTIAL_DEPOSIT;
    pub const TransferFee: Balance = CurrencyId::PONT.millies().times(1);
    pub const CreationFee: Balance = CurrencyId::PONT.millies().times(1);
    pub const TransactionByteFee: Balance = CurrencyId::PONT.millies().times(1);
    // 1 PONT.
    pub const MinVestedTransfer: Balance = CurrencyId::PONT.times(1);
}

impl pallet_vesting::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
    const MAX_VESTING_SCHEDULES: u32 = 1;
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

/// Fees distribution type.
pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);

/// Distribute part of fees to treasury, part burn.
/// Current values - 20% to treasury, the rest - burned.
///
/// Based on Moonbeam implementation - https://github.com/PureStake/moonbeam
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
    R: pallet_balances::Config + pallet_treasury::Config,
    pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
{
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
        if let Some(fees) = fees_then_tips.next() {
            // for fees, 80% are burned, 20% to the treasury
            let (_, to_treasury) = fees.ration(80, 20);
            // Balances pallet automatically burns dropped Negative Imbalances by decreasing
            // total_supply accordingly
            <pallet_treasury::Pallet<R> as OnUnbalanced<_>>::on_unbalanced(to_treasury);
        }
    }
}

parameter_types! {
    pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction =
        pallet_transaction_payment::CurrencyAdapter<Balances, DealWithFees<Runtime>>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
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
    /// Minimum round length is 2 minutes (10 * 12 second block times)
    pub const MinBlocksPerRound: u32 = 10;
    /// Default BlocksPerRound is every hour (300 * 12 second block times)
    pub const DefaultBlocksPerRound: u32 = 300;
    /// Collator candidate exits are delayed by 2 hours (2 * 300 * block_time)
    pub const LeaveCandidatesDelay: u32 = 2;
    /// Nominator exits are delayed by 2 hours (2 * 300 * block_time)
    pub const LeaveNominatorsDelay: u32 = 2;
    /// Nomination revocations are delayed by 2 hours (2 * 300 * block_time)
    pub const RevokeNominationDelay: u32 = 2;
    /// Reward payments are delayed by 2 hours (2 * 300 * block_time)
    pub const RewardPaymentDelay: u32 = 2;
    /// Minimum 8 collators selected per round, default at genesis and minimum forever after
    pub const MinSelectedCandidates: u32 = 8;
    /// Maximum 10 nominators per collator
    pub const MaxNominatorsPerCollator: u32 = 10;
    /// Maximum 25 collators per nominator
    pub const MaxCollatorsPerNominator: u32 = 25;
    /// Default fixed percent a collator takes off the top of due rewards is 20%
    pub const DefaultCollatorCommission: Perbill = Perbill::from_percent(20);
    /// Default percent of inflation set aside for parachain bond every round
    pub const DefaultParachainBondReservePercent: Percent = Percent::from_percent(30);
    /// Minimum stake required to become a collator is 1_000
    pub const MinCollatorStk: Balance = CurrencyId::PONT.times(1000);
    /// Minimum stake required to be reserved to be a candidate is 100
    pub const MinCollatorCandidateStk: Balance = CurrencyId::PONT.times(100);
    /// Minimum stake required to be reserved to be a nominator is 5
    pub const MinNominatorStk: Balance = CurrencyId::PONT.times(1);
}
impl parachain_staking::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type MonetaryGovernanceOrigin = EnsureRoot<AccountId>;
    type MinBlocksPerRound = MinBlocksPerRound;
    type DefaultBlocksPerRound = DefaultBlocksPerRound;
    type LeaveCandidatesDelay = LeaveCandidatesDelay;
    type LeaveNominatorsDelay = LeaveNominatorsDelay;
    type RevokeNominationDelay = RevokeNominationDelay;
    type RewardPaymentDelay = RewardPaymentDelay;
    type MinSelectedCandidates = MinSelectedCandidates;
    type MaxNominatorsPerCollator = MaxNominatorsPerCollator;
    type MaxCollatorsPerNominator = MaxCollatorsPerNominator;
    type DefaultCollatorCommission = DefaultCollatorCommission;
    type DefaultParachainBondReservePercent = DefaultParachainBondReservePercent;
    type MinCollatorStk = MinCollatorStk;
    type MinCollatorCandidateStk = MinCollatorCandidateStk;
    type MinNomination = MinNominatorStk;
    type MinNominatorStk = MinNominatorStk;
    type WeightInfo = parachain_staking::weights::SubstrateWeight<Runtime>;
}

// The pallet connect authors mapping, slots, and implement block executor for nimbus consensus.
impl pallet_author_inherent::Config for Runtime {
    type AuthorId = NimbusId;
    type SlotBeacon = RelaychainBlockNumberProvider<Self>;
    type AccountLookup = AuthorMapping;
    type EventHandler = ParachainStaking;
    type CanAuthor = AuthorFilter;
}

parameter_types! {
    pub const DepositAmount: Balance = CurrencyId::PONT.times(1);
}
// This is a simple session key manager. It should probably either work with, or be replaced
// entirely by pallet sessions.
// We need author mapping to connect Nimbus Ids with Account Ids, all collators should register his AuthorId.
impl pallet_author_mapping::Config for Runtime {
    type Event = Event;
    type AuthorId = NimbusId;
    type DepositCurrency = Balances;
    type DepositAmount = DepositAmount;
    type WeightInfo = pallet_author_mapping::weights::SubstrateWeight<Runtime>;
}

// Filter slots between collators (in nutshell author slot filter pallet chooses who's producer for each slot).
impl pallet_author_slot_filter::Config for Runtime {
    type Event = Event;
    type RandomnessSource = RandomnessCollectiveFlip;
    type PotentialAuthors = ParachainStaking;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

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
    UnknownTokens,
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

parameter_types! {
    pub const MultisigCostPerSig: Balance = 500;
    pub const MultisigCostPerFact: Balance = 500;
    pub const MaxSigners: u16 = 16;
}

impl pallet_multisig::Config for Runtime {
    type Event = Event;

    type Call = Call;

    type MyOrigin = Origin;

    type Currency = Balances;

    type MaxSignatories = MaxSigners;

    type DepositBase = MultisigCostPerSig;

    type DepositFactor = MultisigCostPerFact;

    type WeightInfo = ();
}

/// Move VM similar to Ethereum utilizing gas approach.
/// The gas in case of Pontem and Move VM has been done in a similar way to Moonbeam EVM approach.
/// To get gas to weight conversion we are doing ratio: `WEIGHT_PER_GAS = WEIGHT_PER_SECOND / GAS_PER_SECOND`.
/// To estimate `GAS_PER_SECOND` we used benchmarks we have done in Move VM.
/// Benchmarks: https://github.com/pontem-network/sp-move-vm/blob/1.5/mvm/tests/gas_bench.rs (ignored `read_write_loop`).
/// Benchmarks runned on the instance: 8GB RAM, 4 core CPU, SSD.
/// The value `GAS_PER_SECOND` is not final, and can be changed later after tests on production.
/// So currently max gas is `GAS_PER_SECOND * MAXIMUM_BLOCK_WEIGHT * NORMAL_DISPATCH_RATIO`.
/// IMPORTANT: take into account you also paying gas for transaction bytes, include Move VM module/tx bytes, so really final max gas is different
/// for each transaction because it also depends on the size of assets.
pub const GAS_PER_SECOND: u64 = 6_500_000;

/// Weight / gas ratio.
/// Could be used to convert weight to gas and gas to weight.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND / GAS_PER_SECOND;

pub struct MoveVMGasWeightMapping;

/// Trait `GasWeightMapping` implementation for `MoveVMGasWeightMapping`.
/// Converting gas to weight and weight to gas.
impl GasWeightMapping for MoveVMGasWeightMapping {
    fn gas_to_weight(gas: u64) -> Weight {
        gas.saturating_mul(WEIGHT_PER_GAS)
    }

    fn weight_to_gas(weight: Weight) -> u64 {
        u64::try_from(weight.wrapping_div(WEIGHT_PER_GAS)).unwrap_or(u32::MAX as u64)
    }
}

parameter_types! {
    /// VM pallet address (used to reserve funds during VM native operations).
    pub const MVMPalletId: PalletId = PalletId(*b"pont/mvm");
}

/// Configure the Move-pallet in pallets/sp-mvm.
impl sp_mvm::Config for Runtime {
    /// Events.
    type Event = Event;

    /// Gas weight mapping.
    type GasWeightMapping = MoveVMGasWeightMapping;

    /// Only sudo can deploy modules under 0x or update standard library.
    type UpdaterOrigin = EnsureRoot<AccountId>;

    /// Pallet Id.
    type PalletId = MVMPalletId;

    /// Currency id.
    type CurrencyId = CurrencyId;

    /// Currencies (Multicurrency).
    type Currencies = Currencies;

    /// Weight information.
    type WeightInfo = ();
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

pub fn dollar(currency_id: CurrencyId) -> u128 {
    10u128.pow(currency_id.decimals().into())
}

pub struct CurrencyIdConvert;
impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
    fn convert(id: CurrencyId) -> Option<MultiLocation> {
        match id {
            CurrencyId::KSM => Some(MultiLocation::parent()),
            CurrencyId::PONT => Some(
                (
                    Parent,
                    Junction::Parachain(ParachainInfo::get().into()),
                    Junction::GeneralKey(CurrencyId::PONT.symbol()),
                )
                    .into(),
            ),
        }
    }
}

impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
    fn convert(location: MultiLocation) -> Option<CurrencyId> {
        if location == MultiLocation::parent() {
            return Some(CurrencyId::KSM);
        }

        match location {
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(_id), GeneralKey(key)),
            } if key.to_vec() == CurrencyId::PONT.symbol() => Some(CurrencyId::PONT),
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
    pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
        match currency_id {
            CurrencyId::PONT => PONT_EXISTENTIAL_DEPOSIT,
            CurrencyId::KSM  => KSM_EXISTENTIAL_DEPOSIT
        }
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

impl orml_unknown_tokens::Config for Runtime {
    type Event = Event;
}

parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = CurrencyId::PONT;
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

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = generic::Block<Header, sp_runtime::OpaqueExtrinsic>,
        UncheckedExtrinsic = UncheckedExtrinsic
    {

        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>},

        ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Config, Storage, Inherent, Event<T>} = 20,
        ParachainInfo: parachain_info::{Pallet, Storage, Config} = 21,

        // Balances.
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 30,
        Vesting: pallet_vesting::{Pallet, Call, Storage, Config<T>, Event<T>},
        Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
        Currencies: module_currencies::{Pallet, Call, Storage, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage},

        // Staking.
        ParachainStaking: parachain_staking::{Pallet, Call, Storage, Event<T>, Config<T>} = 40,
        AuthorInherent: pallet_author_inherent::{Pallet, Call, Storage, Inherent} = 41,
        AuthorFilter: pallet_author_slot_filter::{Pallet, Call, Storage, Event, Config} = 42,
        AuthorMapping: pallet_author_mapping::{Pallet, Call, Config<T>, Storage, Event<T>} = 43,

        // Democracy
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Config, Event<T>} = 50,
        Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>} = 51,
        Democracy: pallet_democracy::{Pallet, Call, Storage, Config<T>, Event<T>} = 52,

        // XCM helpers
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 60,
        PolkadotXcm: pallet_xcm::{Pallet, Config, Call, Event<T>, Storage, Origin} = 61,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Call, Event<T>, Origin} = 62,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 63,
        Xtokens: orml_xtokens::{Pallet, Storage, Call, Event<T>} = 64,
        UnknownTokens: orml_unknown_tokens::{Pallet, Storage, Event} = 65,
        OrmlXcm: orml_xcm::{Pallet, Call, Event<T>} = 66,

        // Move VM
        Mvm: sp_mvm::{Pallet, Call, Storage, Config<T>, Event<T>},
        MultiSig: pallet_multisig::{Pallet, Call, Origin<T>, Storage, Event<T>},
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPallets,
>;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info() -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info()
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

    impl sp_mvm_rpc_runtime::MVMApiRuntime<Block, AccountId> for Runtime {
        // Convert Weight to Gas.
        fn gas_to_weight(gas_limit: u64) -> Weight {
             <Runtime as sp_mvm::Config>::GasWeightMapping::gas_to_weight(gas_limit)
        }

        // Convert Gas to Weight.
        fn weight_to_gas(weight: Weight) -> u64 {
            <Runtime as sp_mvm::Config>::GasWeightMapping::weight_to_gas(weight)
        }

        // Estimate gas for publish module.
        fn estimate_gas_publish(account: AccountId, module_bc: Vec<u8>, gas_limit: u64) -> Result<MVMApiEstimation, sp_runtime::DispatchError> {
            // TODO: pass real error.
            let vm_result = Mvm::raw_publish_module(&account, module_bc, gas_limit, true)?;

            Ok(MVMApiEstimation {
                gas_used: vm_result.gas_used,
                status_code: vm_result.status_code as u64,
            })
        }

        // Estimate gas for execute script.
        fn estimate_gas_execute(account: AccountId, tx_bc: Vec<u8>, gas_limit: u64) -> Result<MVMApiEstimation, sp_runtime::DispatchError> {
            let vm_result = Mvm::raw_execute_script(&[account], tx_bc, gas_limit, true)?;

            Ok(MVMApiEstimation {
                gas_used: vm_result.gas_used,
                status_code: vm_result.status_code as u64,
            })
        }

        // Get module binary by it's address
        fn get_module(module_id: Vec<u8>) -> Result<Option<Vec<u8>>, Vec<u8>> {
            Mvm::get_module(&module_id.as_slice())
        }

        // Get module ABI by it's address
        fn get_module_abi(module_id: Vec<u8>) -> Result<Option<Vec<u8>>, Vec<u8>> {
            Mvm::get_module_abi(&module_id.as_slice())
        }

        // Get resource
        fn get_resource(
            account_id: AccountId,
            tag: Vec<u8>,
        ) -> Result<Option<Vec<u8>>, Vec<u8>> {
            Mvm::get_resource(&account_id, &tag.as_slice())
        }

    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }

        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }
    }

    impl nimbus_primitives::AuthorFilterAPI<Block, NimbusId> for Runtime {
        fn can_author(
            author: NimbusId,
            slot: u32,
            parent_header: &<Block as BlockT>::Header
        ) -> bool {
            // The Moonbeam runtimes use an entropy source that needs to do some accounting
            // work during block initialization. Therefore we initialize it here to match
            // the state it will be in when the next block is being executed.
            use frame_support::traits::OnInitialize;
            use nimbus_primitives::CanAuthor;

            System::initialize(
                &(parent_header.number + 1),
                &parent_header.hash(),
                &parent_header.digest,
                frame_system::InitKind::Inspection
            );
            RandomnessCollectiveFlip::on_initialize(System::block_number());

            // And now the actual prediction call
            AuthorInherent::can_author(&author, &slot)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;

            let mut list = Vec::<BenchmarkList>::new();

            list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
            list_benchmark!(list, extra, pallet_balances, Balances);
            list_benchmark!(list, extra, pallet_timestamp, Timestamp);
            list_benchmark!(list, extra, pallet_vesting, Vesting);
            list_benchmark!(list, extra, sp_mvm, Mvm);

            let storage_info = AllPalletsWithSystem::storage_info();

            return (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use frame_system_benchmarking::Pallet as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);
            add_benchmark!(params, batches, pallet_vesting, Vesting);
            add_benchmark!(params, batches, sp_mvm, Mvm);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

cumulus_pallet_parachain_system::register_validate_block!(
    Runtime = Runtime,
    BlockExecutor = pallet_author_inherent::BlockExecutor::<Runtime, Executive>,
    CheckInherents = CheckInherents,
);
