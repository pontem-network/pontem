use frame_support::dispatch;
use sp_runtime::DispatchError;

mod common;
use common::assets::*;
use common::mock::*;
use common::addr::*;

/// Publish module then return result
fn call_publish_module(
    origin: Origin,
    bc: Vec<u8>,
    gas_limit: u64,
) -> dispatch::DispatchResultWithPostInfo {
    let result = Mvm::publish_module(origin, bc, gas_limit);
    eprintln!("publish_module result: {:?}", result);

    result
}

/// Execute script then return result
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

/// Check status == out of gas
fn check_out_of_gas(error: u8, message: Option<&'static str>) {
    assert_eq!(error, 148); // OutOfGas
    assert_eq!(message, Some("OutOfGas"));
}

#[test]
fn publish_std_gas_limit() {
    new_test_ext().execute_with(|| {
        const GAS_LIMIT: u64 = 1;
        let root = root_ps_acc();
        let signer = Origin::signed(root);

        let res = call_publish_module(signer, StdMod::DiemBlock.bc().to_vec(), GAS_LIMIT);

        assert!(res.is_err());

        let err = res.unwrap_err();

        if let DispatchError::Module {
            index: _,
            error,
            message,
        } = err.error
        {
            check_out_of_gas(error, message)
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
            index: _,
            error,
            message,
        } = err.error
        {
            check_out_of_gas(error, message)
        } else {
            panic!("unknown error")
        }
    });
}

#[test]
fn execute_gas_limit() {
    new_test_ext().execute_with(|| {
        const GAS_LIMIT: u64 = 500_000;

        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);

        let res = call_execute_script(signer, UserTx::InfLoop, GAS_LIMIT);

        assert!(res.is_err());

        let err = res.unwrap_err();

        if let DispatchError::Module {
            index: _,
            error,
            message,
        } = err.error
        {
            check_out_of_gas(error, message)
        } else {
            panic!("unknown error");
        }
    });
}
