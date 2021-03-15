use frame_support::weights::Weight;

/// A mapping function that converts Move VM gas to Substrate weight
pub trait GasWeightMapping {
    fn gas_to_weight(gas: u64) -> Weight;
    fn weight_to_gas(weight: Weight) -> u64;
}
