/// Tests related to gas and out of gas cases.
use sp_runtime::{DispatchError, ModuleError};

mod common;
use common::assets::{modules, transactions};
use common::mock::*;
use common::addr::*;
use common::utils;

/// Out of gas error code.
const OUT_OF_GAS_ERROR_CODE: u8 = 150;

/// Minimum gas limit.
const MINIMAL_GAS_LIMIT: u64 = 1;

/// Check status == out of gas.
fn check_out_of_gas(error: DispatchError) {
    if let DispatchError::Module(ModuleError { error, message, .. }) = error {
        assert_eq!(error, OUT_OF_GAS_ERROR_CODE); // OutOfGas
        assert_eq!(message, Some("OutOfGas"));
    } else {
        panic!("Unexpected error: {:?}", error);
    }
}

#[test]
/// Check publish module as root go out of gas limit.
fn publish_gas_limit_as_root() {
    RuntimeBuilder::new().build().execute_with(|| {
        let res =
            utils::publish_module_as_root(&modules::user::EVENT_PROXY, Some(MINIMAL_GAS_LIMIT));

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}

#[test]
/// Check publish module go out of gas limit.
fn publish_gas_limit() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();

        let res = utils::publish_module(origin, &modules::user::STORE, Some(MINIMAL_GAS_LIMIT));

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}

#[test]
/// Check script execution go out of gas limit.
fn execute_gas_limit() {
    RuntimeBuilder::new().build().execute_with(|| {
        const GAS_LIMIT: u64 = 500_000;

        let origin = bob_public_key();

        let res = common::utils::execute_tx(origin, &transactions::INF_LOOP, Some(GAS_LIMIT));

        let error = res.unwrap_err().error;
        check_out_of_gas(error);
    });
}
