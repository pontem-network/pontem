use std::convert::TryInto;

use crate::{
    mock::{*, self},
    utils::generate_preimage,
};
use frame_support::{
    assert_err_ignore_postinfo, assert_ok, dispatch::PostDispatchInfo, weights::Pays,
};
use sp_core::{sr25519};
use sp_keystore::{SyncCryptoStore, testing::KeyStore};
use sp_runtime::{key_types};

#[test]
fn zero_signature_test() {
    let keystore = KeyStore::default();
    let key_a = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Alice"))
        .expect("Generated key");
    let key_b = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Bob"))
        .expect("Generated key");
    let key_c = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Eve"))
        .expect("Generated key");

    new_test_ext().execute_with(move || {
        let f = crate::Pallet::<mock::Test>::groupsign_call(
            mock::Origin::signed(key_a),
            Box::new(Call::System(frame_system::Call::remark {
                remark: b"That's a remark.".to_vec(),
            })),
            vec![key_a, key_b, key_c],
            vec![],
            0,
            10000,
        );
        assert_err_ignore_postinfo!(f, crate::Error::<mock::Test>::ZeroSignatureCall)
    });
}

#[test]
fn incorrect_signature_test() {
    let keystore = KeyStore::default();
    let key_a = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Alice"))
        .expect("Generated key");
    let key_b = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Bob"))
        .expect("Generated key");
    let key_c = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Eve"))
        .expect("Generated key");

    let signatures = keystore
        .sign_with_all(
            key_types::ACCOUNT,
            vec![key_a.into(), key_b.into(), key_c.into()],
            b"Incorrect signature",
        )
        .expect("Successful signing")
        .iter()
        .map(|f| {
            f.as_ref()
                .expect("ref hell ring #1")
                .as_ref()
                .expect("ref hell ring #2")
        })
        .map(|raw_sig| {
            sr25519::Signature(raw_sig.as_slice().try_into().expect("ref hell ring #3")).into()
        })
        .collect::<Vec<_>>();

    new_test_ext().execute_with(move || {
        let f = crate::Pallet::<mock::Test>::groupsign_call(
            mock::Origin::signed(key_a),
            Box::new(Call::System(frame_system::Call::remark {
                remark: b"NixOS is nice and you should use it.".to_vec(),
            })),
            vec![key_a, key_b, key_c],
            signatures,
            0,
            10000,
        );
        assert_err_ignore_postinfo!(f, crate::Error::<mock::Test>::SignatureVerificationError)
    });
}

#[test]
fn incorrect_era_test() {
    let keystore = KeyStore::default();
    let key_a = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Alice"))
        .expect("Generated key");
    let key_b = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Bob"))
        .expect("Generated key");
    let key_c = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Eve"))
        .expect("Generated key");

    let call = Call::System(frame_system::Call::remark {
        remark: b"I want some tasty cookies after that.".to_vec(),
    });

    let signatures = keystore
        .sign_with_all(
            key_types::ACCOUNT,
            vec![key_a.into(), key_b.into(), key_c.into()],
            b"Incorrect signature",
        )
        .expect("Successful signing")
        .iter()
        .map(|f| {
            f.as_ref()
                .expect("ref hell ring #1")
                .as_ref()
                .expect("ref hell ring #2")
        })
        .map(|raw_sig| {
            sr25519::Signature(raw_sig.as_slice().try_into().expect("ref hell ring #3")).into()
        })
        .collect::<Vec<_>>();

    new_test_ext().execute_with(move || {
        let f = crate::Pallet::<mock::Test>::groupsign_call(
            mock::Origin::signed(key_a),
            Box::new(call),
            vec![key_a, key_b, key_c],
            signatures,
            10,
            10000,
        );
        assert_err_ignore_postinfo!(f, crate::Error::<mock::Test>::EraValidationError)
    });
}

#[test]
fn correct_call_test() {
    let keystore = KeyStore::default();
    let key_a = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Alice"))
        .expect("Generated key");
    let key_b = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Bob"))
        .expect("Generated key");
    let key_c = keystore
        .sr25519_generate_new(key_types::ACCOUNT, Some("//Eve"))
        .expect("Generated key");

    let call = Call::System(frame_system::Call::remark {
        remark: b"NixOS is nice and you should use it.".to_vec(),
    });

    new_test_ext().execute_with(move || {
        let preimage = generate_preimage::<mock::Test>(
            &key_a.into(),
            &call,
            &vec![key_a.into(), key_b.into(), key_c.into()],
            10,
            10000,
        );

        let signatures = keystore
            .sign_with_all(
                key_types::ACCOUNT,
                vec![key_a.into(), key_b.into(), key_c.into()],
                &preimage,
            )
            .expect("Successful signing")
            .iter()
            .map(|f| {
                f.as_ref()
                    .expect("ref hell ring #1")
                    .as_ref()
                    .expect("ref hell ring #2")
            })
            .map(|raw_sig| {
                sr25519::Signature(raw_sig.as_slice().try_into().expect("ref hell ring #3"))
                    .into()
            })
            .collect::<Vec<_>>();

        System::set_block_number(123);

        let f = crate::Pallet::<mock::Test>::groupsign_call(
            mock::Origin::signed(key_a),
            Box::new(call),
            vec![key_a, key_b, key_c],
            signatures,
            10,
            10000,
        );
        assert_ok!(
            f,
            PostDispatchInfo {
                actual_weight: None,
                pays_fee: Pays::Yes
            }
        )
    });
}

#[test]
fn bench_groupsign() {}
