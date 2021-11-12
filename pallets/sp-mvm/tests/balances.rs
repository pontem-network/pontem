use frame_support::assert_ok;
use serde::Deserialize;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use move_core_types::account_address::AccountAddress;

mod common;
use common::assets::{modules, transactions};
use common::mock::*;
use common::addr::*;
use common::utils::*;
use test_env_log::test;

use orml_traits::MultiCurrency;

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
        module: Identifier::new(modules::user::STORE.name()).unwrap(),
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
        module: Identifier::new(modules::user::STORE.name()).unwrap(),
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
        publish_module(account, &modules::user::STORE, None).unwrap();

        // execute tx:
        let result = execute_tx(account, &transactions::STORE_NATIVE_BALANCE, None);
        assert_ok!(result);

        // check storage:
        check_storage_u64(to_move_addr(account), INITIAL_BALANCE);

        let balance = balances::Pallet::<Test>::free_balance(&account);
        assert_eq!(INITIAL_BALANCE, balance);
    });
}

#[test]
fn execute_get_token_balance() {
    new_test_ext().execute_with(|| {
        let account = origin_ps_acc();
        let to_deposit = INITIAL_BALANCE;
        let currency = CurrencyId::KSM;

        // Deposit KSM tokens.
        assert_ok!(orml_tokens::Pallet::<Test>::deposit(
            currency, &account, to_deposit
        ));

        // publish user module:
        publish_module(account, &modules::user::STORE, None).unwrap();

        // execute tx:
        let result = execute_tx(account, &transactions::STORE_TOKEN_BALANCE, None);
        assert_ok!(result);

        // check storage:
        check_storage_u64(to_move_addr(account), to_deposit);

        let balance = orml_tokens::Pallet::<Test>::free_balance(currency, &account);
        assert_eq!(to_deposit, balance);
    });
}

#[test]
fn execute_transfer() {
    new_test_ext().execute_with(|| {
        let bob = origin_ps_acc();
        let alice_account = alice_public_key();

        let bob_init_balance = balances::Pallet::<Test>::free_balance(&bob);
        eprintln!("Bob balance: {}", bob_init_balance);

        // publish user module
        publish_module(bob, &modules::user::STORE, None).unwrap();

        // execute tx:
        let result = execute_tx(bob, &transactions::TRANSFER, None);
        assert_ok!(result);

        // check storage balance:
        check_storage_u64(to_move_addr(bob), INITIAL_BALANCE - 2000);

        // check bob balance after script
        let bob_balance = balances::Pallet::<Test>::free_balance(&bob);
        assert_eq!(bob_init_balance - 2000, bob_balance);

        // check alice balance after script
        let alice_balance = balances::Pallet::<Test>::free_balance(&alice_account);
        assert_eq!(INITIAL_BALANCE + 2000, alice_balance);
    });
}

#[test]
fn execute_token_transfer() {
    new_test_ext().execute_with(|| {
        let bob = origin_ps_acc();
        let alice_account = alice_public_key();
        let currency = CurrencyId::KSM;
        let to_deposit = INITIAL_BALANCE;
        let to_transfer = 2000;

        assert_ok!(orml_tokens::Pallet::<Test>::deposit(
            currency, &bob, to_deposit
        ));

        let bob_init_balance = orml_tokens::Pallet::<Test>::free_balance(currency, &bob);
        eprintln!("Bob balance: {}", bob_init_balance);

        // publish user module
        publish_module(bob, &modules::user::STORE, None).unwrap();

        // execute tx:
        let result = execute_tx(bob, &transactions::TRANSFER_TOKEN, None);
        assert_ok!(result);

        // check storage balance:
        check_storage_u64(to_move_addr(bob), INITIAL_BALANCE - to_transfer);

        // check bob balance after script
        let bob_balance = orml_tokens::Pallet::<Test>::free_balance(currency, &bob);
        assert_eq!(bob_init_balance - to_transfer, bob_balance);

        // check alice balance after script
        let alice_balance = orml_tokens::Pallet::<Test>::free_balance(currency, &alice_account);
        assert_eq!(to_transfer, alice_balance);
    });
}

mod adapter {
    use move_vm::io::traits::BalanceAccess;
    use sp_mvm::balance::BalancesAdapter;
    use sp_mvm::balance::boxed::BalancesAdapter as BoxedBalancesAdapter;
    use frame_support::traits::tokens::currency::Currency;
    use orml_traits::MultiCurrency;

    use super::*;
    use test_env_log::test;

    pub type AccountId = <Test as frame_system::Config>::AccountId;

    fn test_get_balance_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let currency = CurrencyId::PONT;
            let expected = balances::Pallet::<Test>::free_balance(&origin);

            let value = adapter.get_balance(&account, currency.symbol().as_ref());
            assert_eq!(Some(expected), value);

            let missed_balance = adapter.get_balance(&account, "".as_bytes());
            assert_eq!(missed_balance, None);
        });
    }

    fn test_get_token_balance_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let to_deposit = 5000;
            let currency = CurrencyId::KSM;

            assert_ok!(orml_tokens::Pallet::<Test>::deposit(
                currency, &origin, to_deposit
            ));

            let expected = orml_tokens::Pallet::<Test>::free_balance(currency, &origin);
            assert_eq!(expected, to_deposit);

            let value = adapter.get_balance(&account, currency.symbol().as_ref());
            assert_eq!(Some(expected), value);
        });
    }

    fn test_sub_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let initial_balance = balances::Pallet::<Test>::free_balance(&origin);
            let currency = CurrencyId::PONT;

            let expected_balance = initial_balance / 2;

            adapter.sub(&account, currency.symbol().as_ref(), expected_balance);

            let actual_balance = balances::Pallet::<Test>::free_balance(&origin);

            assert_eq!(expected_balance, actual_balance);
        });
    }

    fn test_token_sub_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let to_deposit = 5000;
            let currency = CurrencyId::KSM;

            assert_ok!(orml_tokens::Pallet::<Test>::deposit(
                currency, &origin, to_deposit
            ));

            let initial_balance = orml_tokens::Pallet::<Test>::free_balance(currency, &origin);
            assert_eq!(initial_balance, to_deposit);

            let expected_balance = initial_balance / 2;

            adapter.sub(&account, currency.symbol().as_ref(), expected_balance);

            let actual_balance = orml_tokens::Pallet::<Test>::free_balance(currency, &origin);

            assert_eq!(expected_balance, actual_balance);
        });
    }

    fn test_add_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let pallet_account = sp_mvm::Pallet::<Test>::get_account_id();
            let initial_balance = balances::Pallet::<Test>::free_balance(&origin);
            let currency = CurrencyId::PONT;

            let _ = balances::Pallet::<Test>::deposit_creating(&pallet_account, initial_balance);

            adapter.add(&account, currency.symbol().as_ref(), initial_balance);

            let actual_balance = balances::Pallet::<Test>::free_balance(&origin);
            let pallet_actual_balance = balances::Pallet::<Test>::free_balance(&pallet_account);
            assert_eq!(initial_balance * 2, actual_balance);
            assert_eq!(pallet_actual_balance, 0);
        });
    }

    fn test_token_add_with<T: BalanceAccess>(adapter: &T) {
        new_test_ext().execute_with(|| {
            let origin = origin_ps_acc();
            let account = to_move_addr(origin.clone());
            let pallet_account = sp_mvm::Pallet::<Test>::get_account_id();
            let to_deposit = 5000;
            let currency = CurrencyId::KSM;

            assert_ok!(orml_tokens::Pallet::<Test>::deposit(
                currency, &origin, to_deposit
            ));
            assert_ok!(orml_tokens::Pallet::<Test>::deposit(
                currency,
                &pallet_account,
                to_deposit
            ));

            let initial_balance = orml_tokens::Pallet::<Test>::free_balance(currency, &origin);

            adapter.add(&account, currency.symbol().as_ref(), initial_balance);

            let actual_balance = orml_tokens::Pallet::<Test>::free_balance(currency, &origin);
            let pallet_actual_balance =
                orml_tokens::Pallet::<Test>::free_balance(currency, &pallet_account);

            assert_eq!(initial_balance * 2, actual_balance);
            assert_eq!(pallet_actual_balance, 0);
        });
    }

    #[test]
    fn get_balance() {
        let adapter =
            BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get());
        test_get_balance_with(&adapter);
    }

    #[test]
    fn get_token_balance() {
        let adapter =
            BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get());
        test_get_token_balance_with(&adapter);
    }

    #[test]
    fn get_balance_boxed() {
        let adapter =
            BoxedBalancesAdapter::from(
                BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get()),
            );
        test_get_balance_with(&adapter);
    }

    #[test]
    fn get_token_balance_boxed() {
        let adapter =
            BoxedBalancesAdapter::from(
                BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get()),
            );
        test_get_token_balance_with(&adapter);
    }

    #[test]
    fn sub() {
        let adapter =
            BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get());
        test_sub_with(&adapter);
    }

    #[test]
    fn token_sub() {
        let adapter =
            BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get());
        test_token_sub_with(&adapter);
    }

    #[test]
    fn sub_boxed() {
        let adapter =
            BoxedBalancesAdapter::from(
                BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get()),
            );
        test_sub_with(&adapter);
    }

    #[test]
    fn token_sub_boxed() {
        let adapter =
            BoxedBalancesAdapter::from(
                BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get()),
            );
        test_token_sub_with(&adapter);
    }

    #[test]
    fn add() {
        let adapter =
            BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get());
        test_add_with(&adapter);
    }

    #[test]
    fn token_add() {
        let adapter =
            BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get());
        test_token_add_with(&adapter);
    }

    #[test]
    fn add_boxed() {
        let adapter =
            BoxedBalancesAdapter::from(
                BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get()),
            );
        test_add_with(&adapter);
    }

    #[test]
    fn token_add_boxed() {
        let adapter =
            BoxedBalancesAdapter::from(
                BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(MVMPalletId::get()),
            );
        test_token_add_with(&adapter);
    }
}
