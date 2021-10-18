#![cfg_attr(not(feature = "std"), no_std)]
// clippy doesn't likes sp- macros
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use sp_std::prelude::*;
use frame_support::weights::Weight;

pub mod types;

// Describe Runtime API for MVM pallet.
sp_api::decl_runtime_apis! {
    pub trait MVMApiRuntime<AccountId> where
        AccountId: codec::Codec,
    {
        // Convert Weight to Gas.
        fn gas_to_weight(gas_limit: u64) -> Weight;

        // Convert Gas to Weight.
        fn weight_to_gas(weight: Weight) -> u64;

        // Estimate gas for publish module.
        fn estimate_gas_publish(account: AccountId, module_bc: Vec<u8>, gas_limit: u64) -> Result<types::MVMApiEstimation, sp_runtime::DispatchError>;

        // Estimate gas for execute script.
        fn estimate_gas_execute(account: AccountId, tx_bc: Vec<u8>, gas_limit: u64) -> Result<types::MVMApiEstimation, sp_runtime::DispatchError>;

        // Get module binary by it's address
        fn get_module(module_id: Vec<u8>) -> Result<Option<Vec<u8>>, Vec<u8>>;

        // Get module ABI by it's address
        fn get_module_abi(module_id: Vec<u8>) -> Result<Option<Vec<u8>>, Vec<u8>>;

        // Get resource
        fn get_resource(account: AccountId, tag: Vec<u8>) -> Result<Option<Vec<u8>>, Vec<u8>>;

    }
}
