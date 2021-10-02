// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0 

//! Basic weight <-> gas trait implementation for Move VM.
//! 
//! Move VM uses a similar gas model to EVM.
//! As we are using Substrate we should allow us to convert gas to weight, and weight to gas.
use frame_support::weights::Weight;

/// A mapping function that converts Move VM gas to Substrate weight.
pub trait GasWeightMapping {
    /// Convert gas to weight.
    fn gas_to_weight(gas: u64) -> Weight;

    /// Convert weight to gas.
    fn weight_to_gas(weight: Weight) -> u64;
}
