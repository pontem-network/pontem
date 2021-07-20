use frame_support::dispatch;
use sp_runtime::DispatchError;

mod common;
use common::assets::*;
use common::mock::*;
use common::addr::*;

const OUT_OF_GAS_ERROR_CODE: u8 = 148;
const MINIMAL_GAS_LIMIT: u64 = 1;

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
fn check_out_of_gas(error: DispatchError) {
    if let DispatchError::Module { error, message, .. } = error {
        assert_eq!(error, OUT_OF_GAS_ERROR_CODE); // OutOfGas
        assert_eq!(message, Some("OutOfGas"));
    } else {
        panic!("Unexpected error: {:?}", error);
    }
}

#[test]
fn publish_module_gas_limit() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let signer = Origin::signed(root);

        let res =
            call_publish_module(signer, UserMod::EventProxy.bc().to_vec(), MINIMAL_GAS_LIMIT);

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}

#[test]
fn publish_gas_limit() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);

        let res = call_publish_module(signer, UserMod::Store.bc().to_vec(), MINIMAL_GAS_LIMIT);

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}

#[test]
fn execute_gas_limit() {
    new_test_ext().execute_with(|| {
        const GAS_LIMIT: u64 = 500_000;

        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);

        let res = call_execute_script(signer, UserTx::InfLoop, GAS_LIMIT);

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}
