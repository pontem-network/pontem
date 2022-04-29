use crate::tests::mock::{
    RuntimeBuilder, Accounts, Origin, TransactionPause, CurrencyId, Currencies, ParachainStaking,
    Perbill,
};
use frame_support::{assert_ok, assert_err, error::BadOrigin};

#[test]
fn ensure_root_in_transaction_pause() {
    RuntimeBuilder::new().build().execute_with(|| {
        assert_ok!(TransactionPause::pause_transaction(
            Origin::root(),
            b"Balances".to_vec(),
            b"transfer".to_vec()
        ));
        assert_err!(
            TransactionPause::pause_transaction(
                Origin::signed(Accounts::BOB.account()),
                b"Balances".to_vec(),
                b"transfer".to_vec()
            ),
            BadOrigin
        );
    })
}

#[test]
fn ensure_root_in_update_balance() {
    RuntimeBuilder::new().build().execute_with(|| {
        assert_err!(
            Currencies::update_balance(
                Origin::signed(Accounts::ALICE.account()),
                Accounts::ALICE.account().into(),
                CurrencyId::NATIVE,
                100
            ),
            BadOrigin
        );
        assert_ok!(Currencies::update_balance(
            Origin::root(),
            Accounts::ALICE.account().into(),
            CurrencyId::NATIVE,
            100
        ));
    });
}

#[test]
fn ensure_root_in_parachain_staking() {
    RuntimeBuilder::new().build().execute_with(|| {
        assert_err!(
            ParachainStaking::set_blocks_per_round(
                Origin::signed(Accounts::ALICE.account()),
                20u32
            ),
            BadOrigin
        );
        assert_ok!(ParachainStaking::set_blocks_per_round(
            Origin::root(),
            42u32
        ));
        assert_err!(
            ParachainStaking::set_collator_commission(
                Origin::signed(Accounts::ALICE.account()),
                Perbill::from_percent(5)
            ),
            BadOrigin
        );
        assert_ok!(ParachainStaking::set_collator_commission(
            Origin::root(),
            Perbill::from_percent(5)
        ));
        assert_err!(
            ParachainStaking::set_total_selected(
                Origin::signed(Accounts::ALICE.account()),
                42u32
            ),
            BadOrigin
        );
        assert_ok!(ParachainStaking::set_total_selected(Origin::root(), 42u32));
    })
}
