use frame_support::{dispatch};
use sp_runtime::DispatchError;

mod common;
use common::assets::*;
use common::mock::*;
use common::utils::*;

// Publish module.
fn call_publish_module(
    origin: Origin,
    bc: Vec<u8>,
    gas_limit: u64,
) -> dispatch::DispatchResultWithPostInfo {
    let result = Mvm::publish_module(origin, bc, gas_limit);
    eprintln!("publish_module result: {:?}", result);

    result
}

// Execute script.
fn call_execute_script(
    origin: Origin,
    tx: UserTx,
    gas_limit: u64,
) -> dispatch::DispatchResultWithPostInfo {
    let txbc = tx.bc().to_vec();

    let result = Mvm::execute(origin, txbc, gas_limit);
    eprintln!("execute_script result: {:?}", result);

    result
}

// Check status == out of gas.
fn check_out_of_gas(index: u8, error: u8, message: Option<&'static str>) {
    assert_eq!(index, 0);
    assert_eq!(error, 148); // OutOfGas.
    assert_eq!(message, Some("OutOfGas"));
}

#[test]
fn publish_std_gas_limit() {
    new_test_ext().execute_with(|| {
        const GAS_LIMIT: u64 = 1;
        let root = root_ps_acc();
        let signer = Origin::signed(root);

        let res = call_publish_module(signer, StdMod::Block.bc().to_vec(), GAS_LIMIT);

        assert!(res.is_err());

        let err = res.unwrap_err();

        if let DispatchError::Module {
            index,
            error,
            message,
        } = err.error
        {
            check_out_of_gas(index, error, message)
        } else {
            panic!("unknown error")
        }
    });
}

#[test]
fn publish_gas_limit() {
    new_test_ext().execute_with(|| {
        const GAS_LIMIT: u64 = 1;
        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);

        let res = call_publish_module(signer, UserMod::Store.bc().to_vec(), GAS_LIMIT);

        assert!(res.is_err());

        let err = res.unwrap_err();

        if let DispatchError::Module {
            index,
            error,
            message,
        } = err.error
        {
            check_out_of_gas(index, error, message)
        } else {
            panic!("unknown error")
        }
    });
}

#[test]
fn execute_gas_limit() {
    new_test_ext().execute_with(|| {
        const GAS_LIMIT: u64 = 100_000;

        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);

        let res = call_execute_script(signer, UserTx::InfLoop, GAS_LIMIT);

        assert!(res.is_err());

        let err = res.unwrap_err();

        if let DispatchError::Module {
            index,
            error,
            message,
        } = err.error
        {
            check_out_of_gas(index, error, message)
        } else {
            panic!("unknown error");
        }
    });
}
/*
#[test]
fn execute_store_block() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);
        let block = StdMod::Block;
        let store = UserMod::Store;

        call_publish_module(root, block.bc().to_vec(), block.name());
        call_publish_module(origin, store.bc().to_vec(), store.name());

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        call_execute_script_tx_block(signer, UserTx::StoreSysBlock);
        check_storage_block(EXPECTED);
    });
}

#[test]
fn execute_store_time() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);
        let time = StdMod::Time;
        let store = UserMod::Store;

        call_publish_module(root, time.bc().to_vec(), time.name());
        call_publish_module(origin, store.bc().to_vec(), store.name());

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        call_execute_script_tx_block(signer, UserTx::StoreSysTime);
        check_storage_block(EXPECTED * TIME_BLOCK_MULTIPLIER);
    });
}
*/
