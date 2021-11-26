use crate::tests::mock::*;
use frame_support::assert_ok;

use sp_runtime::MultiAddress::Id as MultiId;
use orml_traits::currency::MultiCurrency;

#[test]
fn transfer_currency_using_balances() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = 100 * PONT;
    let to_transfer = initial_balance / 2;
    let final_balance = to_transfer;

    RuntimeBuilder::new().set_balances(
        vec![
            (Accounts::ALICE.account(), CurrencyId::PONT, initial_balance)
        ]
    ).build().execute_with(|| {
        assert_eq!(Balances::free_balance(
            Accounts::ALICE.account(),
        ), initial_balance);

        assert_eq!(Tokens::free_balance(
            currency_id,
            &Accounts::ALICE.account()
        ), 0);

        assert_eq!(
            Currencies::free_balance(
                currency_id,
                &Accounts::ALICE.account()
            ),
            initial_balance
        );

        assert_ok!(
            Balances::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                to_transfer,
            )
        );

        assert_eq!(Balances::free_balance(Accounts::ALICE.account()), final_balance);

        assert_eq!(
            Currencies::free_balance(
                currency_id,
                &Accounts::ALICE.account()
            ),
            final_balance
        );

        assert_eq!(Tokens::free_balance(
            currency_id,
            &Accounts::ALICE.account()
        ), 0);
    });
}
