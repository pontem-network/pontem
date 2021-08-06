pub use frame_support::{
    weights::{constants::WEIGHT_PER_SECOND, Weight},
    traits::CurrencyToVote,
};
pub use sp_mvm::gas::GasWeightMapping;

// u64 currency to vote.
pub struct U64CurrencyToVote;

impl CurrencyToVote<u64> for U64CurrencyToVote {
    fn to_vote(value: u64, _issuance: u64) -> u64 {
        value
    }

    fn to_currency(value: u128, _issuance: u64) -> u64 {
        use core::convert::TryFrom;
        u64::try_from(value).unwrap() // The error should never happen as we use u64 as balances everywhere
    }
}

/// By inheritance from Moonbeam and from Dfinance (based on validators statistic), we believe max 4125000 gas is currently enough for block.
/// In the same time we use same 500ms Weight as Max Block Weight, from which 75% only are used for transactions.
/// So our max gas is GAS_PER_SECOND * 0.500 * 0.65 => 4125000.
pub const GAS_PER_SECOND: u64 = 11_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND / GAS_PER_SECOND;

// Gas Weight Mapping.
pub struct MoveVMGasWeightMapping;

// Just use provided gas.
impl GasWeightMapping for MoveVMGasWeightMapping {
    fn gas_to_weight(gas: u64) -> Weight {
        gas.saturating_mul(WEIGHT_PER_GAS)
    }

    fn weight_to_gas(weight: Weight) -> u64 {
        use core::convert::TryFrom;
        u64::try_from(weight.wrapping_div(WEIGHT_PER_GAS)).unwrap_or(u32::MAX as u64)
    }
}
