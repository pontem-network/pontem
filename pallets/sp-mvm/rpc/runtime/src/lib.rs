#![cfg_attr(not(feature = "std"), no_std)]

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
    }
}
