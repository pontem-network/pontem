//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Groupsign;
use frame_benchmarking::{benchmarks};
use frame_system::{RawOrigin, };
use codec::Decode;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

#[derive(Decode, Clone)]
pub struct TestCase<T> where T : Config {
    call: <T as Config>::Call,
    caller: T::AccountId,
    signatures: Vec<<T as Config>::Signature>,
    signers: Vec<T::AccountId>,
}

benchmarks! {
    groupsign_call {
        let benchmarks = include_bytes!("../benchmark_examples.codec");
        // These tests are generated in build.rs. Refer to build.rs, if you want to change anything in here.
        let tests = Vec::<TestCase<T>>::decode(&mut &benchmarks[..]).expect("Failed to decode test data");
        let s in 0 .. 120;

        let TestCase { call, caller, signatures, signers } = tests.get(s as usize).unwrap().clone();
        let valid_since = <T::BlockNumber as Bounded>::min_value();
        let valid_thru = <T::BlockNumber as Bounded>::max_value();

    }: _(RawOrigin::Signed(caller), Box::new(call), signers, signatures, valid_since, valid_thru)
    verify {

        // assert_eq!(Something::<T>::get(), Some(s));
    }

    impl_benchmark_test_suite!(Groupsign, crate::mock::new_test_ext(), crate::mock::Test);
}
