#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// temporary placeholder for auto generated weights

/// Weight functions needed for groupsign pallet.
pub trait WeightInfo {
	fn groupsign_call(signatures: u32, call_length: u32) -> Weight;
}

/// Just like SubstrateWeights, but measured in Pontem.
pub struct PontemWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PontemWeights<T> {
    fn groupsign_call(signatures: u32, call_length: u32) -> Weight {
        (signatures * 42 + call_length * 34).into() // TODO: Needs benches
    }
}
