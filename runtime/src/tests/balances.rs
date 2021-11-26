/// Test balances in Runtime.
use crate::tests::mock::*;
use frame_support::assert_ok;

use sp_runtime::MultiAddress::Id as MultiId;
use orml_traits::{currency::MultiCurrency, GetByKey};

#[test]
// Test correct native currency.
fn test_get_native_currency() {
    assert_eq!(GetNativeCurrencyId::get(), CurrencyId::PONT);
}

#[test]
/// Test existential deposits.
fn test_existential_deposits() {
    assert_eq!(
        ExistentialDeposits::get(&CurrencyId::PONT),
        PONT_EXISTENTIAL_DEPOSIT,
    );

    assert_eq!(
        ExistentialDeposits::get(&CurrencyId::KSM),
        KSM_EXISTENTIAL_DEPOSIT,
    );
}

#[test]
/// Test transfer native currency using Balances pallet.
fn transfer_native_currency_via_balances() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = to_unit(100, currency_id);
    let to_transfer = initial_balance / 2;
    let final_balance = to_transfer;

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::ALICE.account(),
            CurrencyId::PONT,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            assert_eq!(
                Balances::free_balance(Accounts::ALICE.account()),
                initial_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::ALICE.account()),
                0
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_ok!(Balances::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                to_transfer,
            ));

            assert_eq!(
                Balances::free_balance(Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::ALICE.account()),
                0
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::BOB.account()),
                0
            );

            assert_eq!(
                Balances::free_balance(&Accounts::BOB.account()),
                to_transfer
            );
        });
}

#[test]
/// Test native currency transfer using Currencies pallet.
fn transfer_native_currency_via_currencies() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = to_unit(100, currency_id);
    let to_transfer = initial_balance / 2;
    let final_balance = to_transfer;

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::ALICE.account(),
            CurrencyId::PONT,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            assert_eq!(
                Balances::free_balance(Accounts::ALICE.account(),),
                initial_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::ALICE.account()),
                0
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_ok!(Currencies::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                currency_id,
                to_transfer,
            ));

            assert_eq!(
                Balances::free_balance(Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::BOB.account()),
                0
            );

            assert_eq!(
                Balances::free_balance(&Accounts::BOB.account()),
                to_transfer,
            );
        });
}

#[test]
/// Test tokens transfer via Currencies pallet.
fn transfer_tokens_via_currencies() {
    let native_id = GetNativeCurrencyId::get();
    let currency_id = CurrencyId::KSM;

    let native_balance = to_unit(100, native_id);
    let initial_balance = to_unit(100, currency_id);
    let to_transfer = initial_balance / 2;
    let final_balance = to_transfer;

    RuntimeBuilder::new()
        .set_balances(vec![
            (Accounts::ALICE.account(), CurrencyId::PONT, native_balance),
            (Accounts::ALICE.account(), CurrencyId::KSM, initial_balance),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_ok!(Currencies::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                currency_id,
                to_transfer,
            ));

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::BOB.account()),
                to_transfer,
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::BOB.account()),
                to_transfer
            );
        });
}

#[test]
/// Test tokens transfer via tokens pallet.
fn transfer_tokens_via_tokens() {
    let native_id = GetNativeCurrencyId::get();
    let currency_id = CurrencyId::KSM;

    let native_balance = to_unit(100, native_id);
    let initial_balance = to_unit(100, currency_id);
    let to_transfer = initial_balance / 2;
    let final_balance = to_transfer;

    RuntimeBuilder::new()
        .set_balances(vec![
            (Accounts::ALICE.account(), CurrencyId::PONT, native_balance),
            (Accounts::ALICE.account(), CurrencyId::KSM, initial_balance),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_ok!(Tokens::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                currency_id,
                to_transfer,
            ));

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::ALICE.account()),
                final_balance
            );

            assert_eq!(
                Tokens::free_balance(currency_id, &Accounts::BOB.account()),
                to_transfer,
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::BOB.account()),
                to_transfer
            );
        });
}
