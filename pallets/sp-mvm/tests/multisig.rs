mod common;

use common::mock::*;
use common::addr::{alice_public_key, bob_public_key};
use common::assets::transactions;

use sp_mvm::Call as MvmCall;
use frame_support::{assert_ok, dispatch::GetDispatchInfo};
use sp_runtime::codec::Encode;

const GAS_LIMIT: u64 = 1_000_000;

#[test]
fn execute_multisig() {
    new_test_ext().execute_with(|| {
        let alice_key = alice_public_key();
        let bob_key = bob_public_key();

        roll_next_block(); // we need it to emit events

        let now = || Multisig::timepoint();

        let bytecode = transactions::MULTISIG_TEST.bytes().to_vec();
        let call = Call::Mvm(MvmCall::execute {
            tx_bc: bytecode,
            gas_limit: GAS_LIMIT,
        });
        let weight = call.get_dispatch_info().weight;
        let call = call.encode();
        let call_hash = sp_core::blake2_256(&call);

        assert_ok!(Multisig::as_multi(
            Origin::signed(alice_key),
            2,
            vec![bob_key],
            None,
            call.clone(),
            false,
            0
        ));
        assert_ok!(Multisig::as_multi(
            Origin::signed(bob_key),
            2,
            vec![alice_key],
            Some(now()),
            call,
            false,
            weight
        ));

        let mut sorted_signers = vec![alice_key, bob_key];
        sorted_signers.sort();
        let multisig_id = Multisig::multi_account_id(&sorted_signers, 2);
        let expected: Event = pallet_multisig::Event::MultisigExecuted(
            bob_key,
            now(),
            multisig_id,
            call_hash,
            Ok(()),
        )
        .into();

        assert!(Sys::events()
            .iter()
            .find(|event| event.event == expected)
            .is_some());
    });
}
