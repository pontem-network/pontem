//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Groupsign;
use frame_benchmarking::{benchmarks, whitelisted_caller,account};
use frame_system::{RawOrigin, };
use sp_runtime::traits::Bounded;

use utils::*;

benchmarks! {
    groupsign_call {
        let s in 2 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let signers = test_accounts::<T>(0..s);
        let call: <T as Config>::Call = frame_system::Call::<T>::remark { remark: b"bench_remark".to_vec() }.into();
        let valid_since = <T::BlockNumber as Bounded>::min_value();
        let valid_thru = <T::BlockNumber as Bounded>::max_value();
        let signatures = test_sign::<T>(0..s, &generate_preimage::<T>(&caller, &call, &signers, valid_since, valid_thru));

    }: _(RawOrigin::Signed(caller), Box::new(call), signers, signatures.into(), valid_since, valid_thru)
    verify {

        // assert_eq!(Something::<T>::get(), Some(s));
    }

    // impl_benchmark_test_suite!(Groupsign, crate::mock::new_test_ext(), crate::mock::Test);
}
