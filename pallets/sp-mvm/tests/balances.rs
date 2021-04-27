use serde::Deserialize;
use frame_support::assert_ok;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use move_core_types::account_address::AccountAddress;

mod common;
use common::assets::*;
use common::mock::*;
use common::addr::*;
use common::utils::*;

const GAS_LIMIT: u64 = 1_000_000;

#[derive(Deserialize, Debug, PartialEq)]
struct StoreU128 {
    pub val: u128,
}

fn check_storage_u128(address: AccountAddress, expected: u128) {
    let expected = StoreU128 { val: expected };
    let tag = StructTag {
        address,
        module: Identifier::new(UserMod::Store.name()).unwrap(),
        name: Identifier::new("U128").unwrap(),
        type_params: vec![],
    };
    check_storage_res(address, tag, expected);
}

#[test]
fn execute_get_balance() {
    new_test_ext().execute_with(|| {
        let account = origin_ps_acc();

        // publish entire std lib:
        publish_std();

        // publish user module:
        publish_module(account, UserMod::Store);

        // execute tx:
        let signer = Origin::signed(account);
        let result = execute_tx_unchecked(signer, UserTx::StoreGetBalance, GAS_LIMIT);
        assert_ok!(result);

        // check storage:
        check_storage_u128(to_move_addr(account), INITIAL_BALANCE);
        // TODO: chack PS balance
    });
}

#[test]
fn execute_deposit_balance() {
    new_test_ext().execute_with(|| {
        let account = origin_ps_acc();

        // publish entire std lib:
        publish_std();

        // publish user module:
        publish_module(account, UserMod::Store);

        // execute tx:
        let signer = Origin::signed(account);
        let result = execute_tx_unchecked(signer, UserTx::StoreNativeDepositReg, GAS_LIMIT);
        assert_ok!(result);

        // check storage:
        check_storage_u128(to_move_addr(account), INITIAL_BALANCE / 2);
        // TODO: chack PS balance
    });
}

#[test]
fn execute_deposit_withdraw_balance() {
    new_test_ext().execute_with(|| {
        let account = origin_ps_acc();

        // publish entire std lib:
        publish_std();

        // publish user module:
        publish_module(account, UserMod::Store);

        // execute tx:
        let signer = Origin::signed(account);
        let result = execute_tx_unchecked(signer, UserTx::StoreNativeWithdrawReg, GAS_LIMIT);
        assert_ok!(result);

        // check storage:
        check_storage_u128(to_move_addr(account), INITIAL_BALANCE);
        // TODO: chack PS balance
    });
}
