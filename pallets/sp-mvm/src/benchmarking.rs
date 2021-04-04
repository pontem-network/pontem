#![cfg(feature = "runtime-benchmarks")]
//! Benchmarking setup for pallet-template

// How to use:
// 1. Build node with feature `runtime-benchmarks`
// 2. Run `./target/release/mv-node benchmark --dev -lsp_mvm=trace --pallet=sp_mvm --execution=wasm --wasm-execution=compiled --extrinsic='*' --steps=20 --repeat=10 --output=./target/sp-bench/`

use super::*;
use sp_std::prelude::*;
use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};

#[allow(unused)]
use super::Module as Mvm;

benchmarks! {
    do_something {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), s)
    verify {
        assert_eq!(Something::<T>::get(), Some(s));
    }
}

impl_benchmark_test_suite!(Mvm, crate::mock::new_test_ext(), crate::mock::Test,);
