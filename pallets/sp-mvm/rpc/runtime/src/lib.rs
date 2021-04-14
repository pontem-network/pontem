#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::weights::Weight;

sp_api::decl_runtime_apis! {
	pub trait MVMApiRuntime {
		// Convert Weight to Gas.
        fn gas_to_weight(gas_limit: u64) -> Weight;

		// Convert Gas to Weight.
		fn weight_to_gas(weight: Weight) -> u64;
	}
}
