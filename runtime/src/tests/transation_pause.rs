/// Test balances in Runtime.
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_err};

use sp_runtime::MultiAddress::Id as MultiId;
use orml_traits::{currency::MultiCurrency, GetByKey};
use transaction_pause::PausedTransactionFilter;

#[test]
/// Test transfer native currency using Balances pallet.
fn transaction_pause_balance() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = to_unit(100, currency_id);
    let to_transfer = initial_balance / 2;

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::ALICE.account(),
            CurrencyId::PONT,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            assert_ok!(TransactionPause::pause_transaction(
                Origin::root(),
                b"balances".to_vec(),
                b"transfer".to_vec()
            ));
            assert_ok!(Balances::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                to_transfer
            ),);
            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                to_transfer
            );
            assert_ok!(TransactionPause::unpause_transaction(
                Origin::root(),
                b"balances".to_vec(),
                b"transfer".to_vec()
            ));
            assert_ok!(Balances::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                to_transfer,
            ));
            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                0
            );
            System::events().iter().for_each(|ev| eprintln!("{:?}", ev));
        });
}
