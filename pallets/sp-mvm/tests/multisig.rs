mod common;

use common::mock::*;
use common::addr::{alice_public_key, bob_public_key};
use common::assets::transactions;

use sp_mvm::Call as MvmCall;
use frame_support::{assert_ok};
use sp_runtime::codec::Encode;
use sp_core::Pair;
use sp_std::vec;

const GAS_LIMIT: u64 = 1_000_000;

#[test]
fn execute_multisig() {
    RuntimeBuilder::new()
        .set_balances(vec![
            (bob_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
            (alice_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
        ])
        .build()
        .execute_with(|| {
            let alice_key = alice_public_key();
            let bob_key = bob_public_key();

            roll_next_block();

            // Generate transaction call.
            let bytecode = transactions::MULTISIG_TEST.bytes().to_vec();
            let call = Call::Mvm(MvmCall::execute {
                tx_bc: bytecode,
                gas_limit: GAS_LIMIT,
            });

            let since: u64 = 0;
            let till: u64 = 100;
            let nonce: u64 = 0;

            // Generate info to sign.

            // Call groupsign.
            let signers = vec![alice_key, bob_key];

            let mut call_preimage = call.encode();
            call_preimage.extend(since.encode());
            call_preimage.extend(till.encode());
            call_preimage.extend(alice_key.encode());
            call_preimage.extend(nonce.encode());
            call_preimage.extend(signers.encode());

            let to_sign = sp_core::blake2_256(&call_preimage);

            // Generate keypairs.
            let alice_keypair = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
            let alice_signature = alice_keypair.sign(&to_sign[..]);

            let bob_keypair = sp_core::sr25519::Pair::from_string("//Bob", None).unwrap();
            let bob_signature = bob_keypair.sign(&to_sign[..]);

            let signatures = vec![
                AnySignature::from(alice_signature),
                AnySignature::from(bob_signature.clone()),
            ];

            assert_ok!(Groupsign::groupsign_call(
                Origin::signed(alice_key),
                Box::new(call.clone()),
                signers,
                signatures,
                since,
                till,
            ));
        });
}
