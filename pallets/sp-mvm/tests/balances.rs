use frame_support::assert_ok;
use serde::Deserialize;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use move_core_types::account_address::AccountAddress;

mod common;
use common::assets::*;
use common::mock::*;
use common::addr::*;
use common::utils::*;
use test_env_log::test;

#[derive(Deserialize, Debug, PartialEq)]
struct StoreU128 {
    pub val: u128,
}

#[derive(Deserialize, Debug, PartialEq)]
struct StoreU64 {
    pub val: u64,
}

#[allow(dead_code)]
fn check_storage_u128<T>(address: AccountAddress, expected: T)
where
    T: Into<u128>,
{
    let expected = StoreU128 {
        val: expected.into(),
    };
    let tag = StructTag {
        address,
        module: Identifier::new(UserMod::Store.name()).unwrap(),
        name: Identifier::new("U128").unwrap(),
        type_params: vec![],
    };
    check_storage_res(address, tag, expected);
}

fn check_storage_u64<T>(address: AccountAddress, expected: T)
where
    T: Into<u64>,
{
    let expected = StoreU64 {
        val: expected.into(),
    };
    let tag = StructTag {
        address,
        module: Identifier::new(UserMod::Store.name()).unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };
    check_storage_res(address, tag, expected);
}

#[test]
fn execute_get_balance() {
    new_test_ext().execute_with(|| {
        let account = origin_ps_acc();

        // publish user module:
        publish_module(account, UserMod::Store, None).unwrap();

        // execute tx:
        let result = execute_tx(account, UserTx::StoreGetBalance, None);
        assert_ok!(result);

        // check storage:
        check_storage_u64(to_move_addr(account), INITIAL_BALANCE);

        let balance = balances::Pallet::<Test>::free_balance(&account);
        assert_eq!(INITIAL_BALANCE, balance);
    });
}

#[test]
fn execute_transfer() {
    new_test_ext().execute_with(|| {
        let bob = origin_ps_acc();
        let alice_account = alice_acc();

        let bob_init_balance = balances::Pallet::<Test>::free_balance(&bob);
        eprintln!("Bob balance: {}", bob_init_balance);

        // publish user module
        publish_module(bob, UserMod::Store, None).unwrap();

        // execute tx:
        let result = execute_tx(bob, UserTx::Transfer, None);
        assert_ok!(result);

        // check storage balance:
        check_storage_u64(to_move_addr(bob), INITIAL_BALANCE-2000);

        // check bob balance after script
        let bob_balance = balances::Pallet::<Test>::free_balance(&bob);
        assert_eq!(bob_init_balance-2000, bob_balance);

        // check alice balance after script
        let alice_balance = balances::Pallet::<Test>::free_balance(&alice_account);
        assert_eq!(INITIAL_BALANCE+2000, alice_balance);
    });
}

mod adapter {
    use move_vm::io::traits::BalanceAccess;
    use sp_mvm::balance::BalancesAdapter;
    use sp_mvm::balance::boxed::BalancesAdapter as BoxedBalancesAdapter;

    use super::*;
    use test_env_log::test;

    fn test_get_balance_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let expected = balances::Pallet::<Test>::free_balance(&origin);
            let value = adapter.get_balance(&account, "PONT".as_bytes());
            assert_eq!(Some(expected), value);
        });
    }

    fn test_sub_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let initial_balance = balances::Pallet::<Test>::free_balance(&origin);

            let expected_balance = initial_balance / 2;

            adapter.sub(&account, "PONT".as_bytes(), expected_balance);

            let actual_balance = balances::Pallet::<Test>::free_balance(&origin);

            assert_eq!(expected_balance, actual_balance);
        });
    }

    fn test_add_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let initial_balance = balances::Pallet::<Test>::free_balance(&origin);

            adapter.add(&account, "PONT".as_bytes(), initial_balance);

            let actual_balance = balances::Pallet::<Test>::free_balance(&origin);
            assert_eq!(initial_balance * 2, actual_balance);
        });
    }

    #[test]
    fn get_balance() {
        let adapter = BalancesAdapter::<Test>::new();
        test_get_balance_with(&adapter);
    }

    #[test]
    fn get_balance_boxed() {
        let adapter = BoxedBalancesAdapter::from(BalancesAdapter::<Test>::new());
        test_get_balance_with(&adapter);
    }

    #[test]
    fn sub() {
        let adapter = BalancesAdapter::<Test>::new();
        test_sub_with(&adapter);
    }

    #[test]
    fn sub_boxed() {
        let adapter = BoxedBalancesAdapter::from(BalancesAdapter::<Test>::new());
        test_sub_with(&adapter);
    }

    #[test]
    fn add() {
        let adapter = BalancesAdapter::<Test>::new();
        test_add_with(&adapter);
    }

    #[test]
    fn add_boxed() {
        let adapter = BoxedBalancesAdapter::from(BalancesAdapter::<Test>::new());
        test_add_with(&adapter);
    }
}
