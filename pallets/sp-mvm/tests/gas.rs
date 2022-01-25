use sp_runtime::DispatchError;

mod common;
use common::assets::{modules, transactions};
use common::mock::*;
use common::addr::*;
use common::utils;

const OUT_OF_GAS_ERROR_CODE: u8 = 149;
const MINIMAL_GAS_LIMIT: u64 = 1;

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

        let res =
            utils::publish_module(root, &modules::user::EVENT_PROXY, Some(MINIMAL_GAS_LIMIT));

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}

#[test]
fn publish_gas_limit() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();

        let res = utils::publish_module(origin, &modules::user::STORE, Some(MINIMAL_GAS_LIMIT));

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}

#[test]
fn execute_gas_limit() {
    new_test_ext().execute_with(|| {
        const GAS_LIMIT: u64 = 500_000;

        let origin = origin_ps_acc();

        let res = common::utils::execute_tx(origin, &transactions::INF_LOOP, Some(GAS_LIMIT));

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}
