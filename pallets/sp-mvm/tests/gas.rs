use sp_runtime::DispatchError;

mod common;
use common::assets::{modules, transactions};
use common::mock::*;
use common::addr::*;
use common::utils;

const OUT_OF_GAS_ERROR_CODE: u8 = 150;
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
    RuntimeBuilder::new()
        .set_balances(vec![
            (bob_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
            (alice_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
        ])
        .build()
        .execute_with(|| {
            let res = utils::publish_module_as_root(
                &modules::user::EVENT_PROXY,
                Some(MINIMAL_GAS_LIMIT),
            );

            let error = res.unwrap_err().error;
            check_out_of_gas(error);
        });
}

#[test]
fn publish_gas_limit() {
    RuntimeBuilder::new()
        .set_balances(vec![
            (bob_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
            (alice_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
        ])
        .build()
        .execute_with(|| {
            let origin = bob_public_key();

            let res =
                utils::publish_module(origin, &modules::user::STORE, Some(MINIMAL_GAS_LIMIT));

            let error = res.unwrap_err().error;
            check_out_of_gas(error);
        });
}

#[test]
fn execute_gas_limit() {
    RuntimeBuilder::new()
        .set_balances(vec![
            (bob_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
            (alice_public_key(), CurrencyId::NATIVE, INITIAL_BALANCE),
        ])
        .build()
        .execute_with(|| {
            const GAS_LIMIT: u64 = 500_000;

            let origin = bob_public_key();

            let res = common::utils::execute_tx(origin, &transactions::INF_LOOP, Some(GAS_LIMIT));

            let error = res.unwrap_err().error;
            check_out_of_gas(error);
        });
}
