/// Test vesting functional.
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_err};
use sp_runtime::MultiAddress::Id as MultiId;

#[test]
/// Test vesting balances releases correctly.
fn test_vesting_release() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = to_unit(100, CurrencyId::PONT);
    let start_vesting = 10;
    let duration: u32 = 100;
    let free_balance = to_unit(50, CurrencyId::PONT);
    let per_block = (initial_balance - free_balance) / (duration as Balance);

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::ALICE.account(),
            currency_id,
            initial_balance,
        )])
        .set_vesting(vec![(
            Accounts::ALICE.account(),
            start_vesting,
            duration,
            free_balance,
        )])
        .build()
        .execute_with(|| {
            // Check initial state.
            assert_eq!(
                Balances::free_balance(Accounts::ALICE.account()),
                initial_balance,
            );

            assert_eq!(
                Balances::usable_balance(Accounts::ALICE.account()),
                free_balance,
            );

            // Transfer some coins.
            assert_ok!(Balances::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                free_balance,
            ));

            // Check that after transfer usable balance is zero.
            assert_eq!(Balances::usable_balance(Accounts::ALICE.account()), 0,);

            // Check that we can't transfer more.
            assert_err!(
                Balances::transfer(
                    Origin::signed(Accounts::ALICE.account()),
                    MultiId(Accounts::BOB.account()),
                    free_balance
                ),
                pallet_balances::Error::<Runtime>::LiquidityRestrictions
            );

            // Check that nothing released before vesting starts.
            run_to_block(5);

            assert_eq!(Balances::usable_balance(Accounts::ALICE.account()), 0,);

            // Release part of vested balance and transfer to Bob.
            run_to_block(11);
            assert_ok!(Vesting::vest(Origin::signed(Accounts::ALICE.account())));
            assert_eq!(
                Balances::usable_balance(Accounts::ALICE.account()),
                per_block,
            );

            assert_ok!(Balances::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                per_block,
            ));

            // Check that all vested balance released.
            run_to_block(111);
            let expected_balance = per_block * ((duration - 1) as Balance);
            assert_ok!(Vesting::vest(Origin::signed(Accounts::ALICE.account())));
            assert_eq!(
                Balances::usable_balance(Accounts::ALICE.account()),
                expected_balance,
            );

            // Check that balance stopped increasing.
            run_to_block(112);
            assert_err!(
                Vesting::vest(Origin::signed(Accounts::ALICE.account())),
                pallet_vesting::Error::<Runtime>::NotVesting,
            );
        });
}

#[test]
/// Test that currencies also takes vesting into account.
fn test_vesting_via_currencies() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = to_unit(100, currency_id);
    let start_vesting = 10;
    let duration: u32 = 100;
    let free_balance = to_unit(50, currency_id);
    let per_block = (initial_balance - free_balance) / (duration as Balance);

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::ALICE.account(),
            currency_id,
            initial_balance,
        )])
        .set_vesting(vec![(
            Accounts::ALICE.account(),
            start_vesting,
            duration,
            free_balance,
        )])
        .build()
        .execute_with(|| {
            // Check initial state.
            assert_eq!(
                Balances::free_balance(Accounts::ALICE.account()),
                initial_balance,
            );

            let usable_balance = Balances::usable_balance(Accounts::ALICE.account());
            assert_eq!(usable_balance, free_balance,);

            // Check we can't transfer vested funds.
            // Transfer free balance.
            assert_ok!(Currencies::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                currency_id,
                free_balance,
            ));

            // Check that after transfer usable balance is zero.
            assert_eq!(Balances::usable_balance(Accounts::ALICE.account()), 0,);

            // Check that we can't transfer more.
            assert_err!(
                Currencies::transfer(
                    Origin::signed(Accounts::ALICE.account()),
                    MultiId(Accounts::BOB.account()),
                    currency_id,
                    free_balance
                ),
                pallet_balances::Error::<Runtime>::LiquidityRestrictions
            );

            // Release part of vested balance and transfer to Bob.
            run_to_block(11);
            assert_ok!(Vesting::vest(Origin::signed(Accounts::ALICE.account())));
            assert_eq!(
                Balances::usable_balance(Accounts::ALICE.account()),
                per_block,
            );

            assert_ok!(Currencies::transfer(
                Origin::signed(Accounts::ALICE.account()),
                MultiId(Accounts::BOB.account()),
                currency_id,
                per_block,
            ));
        });
}
