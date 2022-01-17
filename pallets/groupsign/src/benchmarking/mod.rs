//! Benchmarking setup for groupsign

use super::*;
pub mod benchlib;
use benchlib::*;

use frame_benchmarking::{benchmarks};
use codec::Decode;
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks! {

    on_chain_message_check {

        // sr25519 signatures
        let a in 0..SIG_STEPS - 1;
        // ed25519 signatures
        let b in 0..SIG_STEPS - 1;
        // ecdsa signatures
        let c in 0..SIG_STEPS - 1;
        // Message lengths
        let d in 0..LEN_STEPS - 1;

        let benchmarks = include_bytes!("benchmark_examples.codec");
        let tests = CannedBenchmarks::decode(&mut &benchmarks[..]).expect("Failed to decode test data");
        let caller: T::AccountId = frame_benchmarking::whitelisted_caller();
        let (message, signers, signatures) = tests.get_by_parameters(a, b, c, d);

    }: on_chain_message_check(RawOrigin::Signed(caller), message, signers.into(), signatures.into())
    verify {}

    impl_benchmark_test_suite!(Groupsign, crate::mock::new_test_ext(), crate::mock::Test);
}
