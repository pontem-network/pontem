//! Benchmarking setup for groupsign

use super::*;

#[allow(unused)]
use crate::Pallet as Groupsign;
use frame_benchmarking::{benchmarks};
use codec::Decode;
use frame_system::RawOrigin;
use sp_std::prelude::*;

#[derive(Decode, Clone)]
pub struct TestCase<T>
where
    T: Config,
{
    message: Vec<u8>,
    signatures: Vec<<T as Config>::Signature>,
    signers: Vec<T::AccountId>,
}

benchmarks! {

    on_chain_message_check {
        let benchmarks = include_bytes!("../benchmark_examples.codec");
        let tests = Vec::<TestCase<T>>::decode(&mut &benchmarks[..]).expect("Failed to decode test data");
        let s in 0 .. 120;

        let TestCase { message, signatures, signers } = tests.get(s as usize).unwrap().clone();
        let caller: T::AccountId = frame_benchmarking::whitelisted_caller();
    }: _(RawOrigin::Signed(caller), message, signers, signatures)
    verify {}

    impl_benchmark_test_suite!(Groupsign, crate::mock::new_test_ext(), crate::mock::Test);
}
