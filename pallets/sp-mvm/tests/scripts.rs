/// Tests related to scripts execution.
use serde::Deserialize;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use frame_support::assert_err_ignore_postinfo;
use frame_support::dispatch::DispatchError;
use sp_mvm::Event;

mod common;
use common::assets::{modules, transactions};
use common::mock::*;
use common::addr::*;
use common::utils;

/// Check stored value (u64) inside storage.
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

    utils::check_storage_res(origin_move_addr(), tag, expected);
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

#[test]
/// Execute script and store U64 number inside storage.
fn execute_script() {
    RuntimeBuilder::new().build().execute_with(|| {
        const EXPECTED: u64 = 42;
        let origin = bob_public_key();

        utils::publish_module(origin, &modules::user::STORE, None).unwrap();
        utils::execute_tx(origin, &transactions::STORE_U64, None).unwrap();

        check_stored_value(EXPECTED);
    });
}

#[test]
/// Execute storing of block height inside module by calling script.
fn execute_store_block() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();

        utils::publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        utils::execute_tx(origin, &transactions::STORE_SYSTEM_BLOCK, None).unwrap();
        check_stored_value(EXPECTED);
    });
}

#[test]
/// Execute storing of timestamp inside module by calling script.
fn execute_store_time() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();

        utils::publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        utils::execute_tx(origin, &transactions::STORE_SYSTEM_TIMESTAMP, None).unwrap();
        check_stored_value(EXPECTED * TIME_BLOCK_MULTIPLIER);
    });
}

#[test]
/// Check the pallet doesn't allow scripts contains root signers.
fn execute_script_as_root() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();
        let result = utils::execute_tx(origin, &transactions::AS_ROOT, None);

        assert_err_ignore_postinfo!(
            result,
            DispatchError::Module {
                index: 6,
                error: 7,
                message: Some("TransactionIsNotAllowedError")
            }
        );
    });
}

#[test]
/// Deploy user module, execute script (storing value in module), check event.
fn execute_script_with_event() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();

        utils::publish_module(origin, &modules::user::EVENT_PROXY, None).unwrap();

        // we need next block because events are not populated on genesis:
        roll_next_block();

        assert!(Sys::events().is_empty());

        utils::execute_tx(origin, &transactions::EMIT_EVENT, None).unwrap();

        // construct event: that should be emitted in the method call directly above
        let tt = TypeTag::Struct(StructTag {
            address: to_move_addr(origin),
            module: Identifier::new(modules::user::EVENT_PROXY.name()).unwrap(),
            name: Identifier::new("U64").unwrap(),
            type_params: Vec::with_capacity(0),
        })
        .to_string();
        let tt = tt.as_bytes();

        // guid is sequence number (8 bytes) followed by account address
        let mut guid = vec![0; 8];
        guid.extend(&origin.0);

        let expected = Event::Event(guid, tt.to_vec(), 42u64.to_le_bytes().to_vec()).into();

        // iterate through array of `EventRecord`s
        assert!(Sys::events().iter().any(|rec| { rec.event == expected }))
    });
}

#[test]
/// Test execution of transaction which __does not__ requires a root/sudo.
/// The Call signied by __ordinar signer__.
fn execute_with_one_signer() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();
        utils::execute_tx(origin, &transactions::ONE_SIGNER_USER, None)
            .expect("tx without root requirement called by ordinar signer");
    });
}

#[test]
/// Test execution of transaction which __requires__ a root/sudo signature.
/// The Call signied by __sudo__.
fn execute_with_one_signer_with_root_by_root() {
    RuntimeBuilder::new().build().execute_with(|| {
        utils::execute_tx_by_root(&transactions::ONE_SIGNER_ROOT, None)
            .expect("tx with root requirement called by root origin");
    });
}

#[test]
#[should_panic(expected = "TransactionIsNotAllowedError")]
/// Test execution of transaction which __does not__ requires a root/sudo.
/// The Call signied by __sudo__.
fn execute_with_one_signer_by_root() {
    RuntimeBuilder::new().build().execute_with(|| {
        let error = utils::execute_tx_by_root(&transactions::ONE_SIGNER_USER, None)
            .expect_err("tx without root requirement called by root signer should fail")
            .error;
        unwrap_move_err_in_dispatch_err(&error);
    });
}

#[test]
#[should_panic(expected = "TransactionIsNotAllowedError")]
/// Test execution of transaction which __requires__ a root/sudo signature.
/// The Call signied by __ordinar signer__, not sudo/root.
fn execute_with_one_signer_with_root() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();
        let error = utils::execute_tx(origin, &transactions::ONE_SIGNER_ROOT, None)
            .expect_err("tx with root requirement called by signed origin should fail")
            .error;
        unwrap_move_err_in_dispatch_err(&error);
    });
}
