use serde::Deserialize;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;

mod common;
use common::assets::{modules, transactions};
use common::mock::*;
use common::addr::*;
use common::utils::*;

fn check_stored_value(expected: u64) {
    #[derive(Deserialize, Debug, PartialEq)]
    struct StoreU64 {
        pub val: u64,
    }

    let expected = StoreU64 { val: expected };

    let tag = StructTag {
        address: origin_move_addr(),
        module: Identifier::new(modules::user::STORE.name()).unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };

    check_storage_res(origin_move_addr(), tag, expected);
}

#[test]
fn execute_store_block() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();

        publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        execute_tx(origin, &transactions::STORE_SYSTEM_BLOCK, None).unwrap();
        check_stored_value(EXPECTED);
    });
}

#[test]
fn execute_store_time() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();

        publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        execute_tx(origin, &transactions::STORE_SYSTEM_TIMESTAMP, None).unwrap();
        check_stored_value(EXPECTED * TIME_BLOCK_MULTIPLIER);
    });
}

#[test]
/// Test execution of transaction.
/// Transaction __does not__ requires a root/sudo.
/// The Call signied by __ordinar signer__.
fn execute_with_one_signer() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();
        execute_tx(origin, &transactions::ONE_SIGNER_USER, None)
            .expect("tx without root requirement called by ordinar signer")
    });
}

#[test]
/// Test execution of transaction.
/// Transaction __requires__ a root/sudo signature.
/// The Call signied by __sudo__.
fn execute_with_one_signer_with_root_by_root() {
    new_test_ext()
        .execute_with(|| execute_tx_by_root(&transactions::ONE_SIGNER_ROOT, None))
        .expect("tx with root requirement called by root origin");
}

#[test]
#[should_panic(expected = "TransactionIsNotAllowedError")]
/// Test execution of transaction.
/// Transaction __does not__ requires a root/sudo.
/// The Call signied by __sudo__.
fn execute_with_one_signer_by_root() {
    let error = new_test_ext()
        .execute_with(|| execute_tx_by_root(&transactions::ONE_SIGNER_USER, None))
        .expect_err("tx without root requirement called by root signer should fail")
        .error;
    unwrap_move_err_in_dispatch_err(&error);
}

#[test]
#[should_panic(expected = "TransactionIsNotAllowedError")]
/// Test execution of transaction.
/// Transaction __requires__ a root/sudo signature.
/// The Call signied by __ordinar signer__, not sudo/root.
fn execute_with_one_signer_with_root() {
    let error = new_test_ext()
        .execute_with(|| {
            let origin = origin_ps_acc();
            execute_tx(origin, &transactions::ONE_SIGNER_ROOT, None)
        })
        .expect_err("tx with root requirement called by ordinar signer should fail")
        .error;
    unwrap_move_err_in_dispatch_err(&error);
}

/// Panics with inner message of the passed error.
/// If message does not exist, just panics with debug render of entire error.
fn unwrap_move_err_in_dispatch_err(err: &sp_runtime::DispatchError) -> ! {
    match err {
        sp_runtime::DispatchError::Module {
            message: Some(message),
            ..
        } => panic!("{}", message),
        _ => panic!("{:?}", err),
    }
}
