/// Test balances in Runtime.
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_err, assert_noop, dispatch::DispatchError};

use sp_runtime::ModuleError;
use sp_runtime::{MultiAddress::Id as MultiId, traits::Dispatchable};
use orml_traits::currency::MultiCurrency;

use frame_system::{EventRecord, Phase};
use transaction_pause::Event as PauseEvent;

#[test]
/// Test transfer native currency using Balances pallet.
fn transaction_pause_balance() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = to_unit(100, currency_id);
    let to_transfer = initial_balance / 2;

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::ALICE.account(),
            CurrencyId::NATIVE,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            let call = <Runtime as frame_system::Config>::Call::Balances(
                pallet_balances::Call::transfer {
                    dest: MultiId(Accounts::BOB.account()),
                    value: to_transfer,
                },
            );

            assert!(<Runtime as frame_system::Config>::BaseCallFilter::contains(
                &call
            ));

            assert_ok!(TransactionPause::pause_transaction(
                Origin::root(),
                b"Balances".to_vec(),
                b"transfer".to_vec()
            ));

            assert!(!<Runtime as frame_system::Config>::BaseCallFilter::contains(&call));

            assert_err!(
                call.clone()
                    .dispatch(Origin::signed(Accounts::ALICE.account())),
                DispatchError::Module(ModuleError {
                    index: 0,
                    error: 5,
                    message: Some("CallFiltered")
                }),
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_ok!(TransactionPause::unpause_transaction(
                Origin::root(),
                b"Balances".to_vec(),
                b"transfer".to_vec()
            ));

            assert_ok!(call.dispatch(Origin::signed(Accounts::ALICE.account())));

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance - to_transfer
            );
            let events = System::events();
            let mut events_iter = events.iter();
            assert_eq!(
                events_iter.next().unwrap(),
                &EventRecord {
                    phase: Phase::Initialization,
                    event: Event::TransactionPause(PauseEvent::TransactionPaused(
                        b"Balances".to_vec(),
                        b"transfer".to_vec()
                    )),
                    topics: [].to_vec()
                }
            );
            assert_eq!(
                events_iter.next().unwrap(),
                &EventRecord {
                    phase: Phase::Initialization,
                    event: Event::TransactionPause(PauseEvent::TransactionUnpaused(
                        b"Balances".to_vec(),
                        b"transfer".to_vec()
                    )),
                    topics: [].to_vec()
                }
            );
        });
}

#[test]
/// Test exlusive root acces to pallet using Balances pallet.
fn transaction_pause_bad_origin() {
    RuntimeBuilder::new().build().execute_with(|| {
        assert_noop!(
            TransactionPause::pause_transaction(
                Origin::signed(Accounts::BOB.account()),
                b"Balances".to_vec(),
                b"transfer".to_vec()
            ),
            BadOrigin
        );

        assert_noop!(
            TransactionPause::unpause_transaction(
                Origin::signed(Accounts::BOB.account()),
                b"Balances".to_vec(),
                b"transfer".to_vec()
            ),
            BadOrigin
        );
    });
}

#[test]
/// Test if we can pause extrinsics via genesis.
fn transaction_pause_genesis() {
    let currency_id = GetNativeCurrencyId::get();

    let initial_balance = to_unit(100, currency_id);
    let to_transfer = initial_balance / 2;

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::ALICE.account(),
            CurrencyId::NATIVE,
            initial_balance,
        )])
        .set_paused(vec![
            (b"Balances".to_vec(), b"transfer".to_vec()),
            (b"Mvm".to_vec(), b"execute".to_vec()),
        ])
        .build()
        .execute_with(|| {
            let mvm_call = <Runtime as frame_system::Config>::Call::Mvm(sp_mvm::Call::execute {
                tx_bc: vec![],
                gas_limit: 100_000,
            });

            assert!(!<Runtime as frame_system::Config>::BaseCallFilter::contains(&mvm_call));

            assert_err!(
                mvm_call
                    .clone()
                    .dispatch(Origin::signed(Accounts::ALICE.account())),
                DispatchError::Module(ModuleError {
                    index: 0,
                    error: 5,
                    message: Some("CallFiltered")
                }),
            );

            let call = <Runtime as frame_system::Config>::Call::Balances(
                pallet_balances::Call::transfer {
                    dest: MultiId(Accounts::BOB.account()),
                    value: to_transfer,
                },
            );

            assert!(!<Runtime as frame_system::Config>::BaseCallFilter::contains(&call));

            assert_err!(
                call.clone()
                    .dispatch(Origin::signed(Accounts::ALICE.account())),
                DispatchError::Module(ModuleError {
                    index: 0,
                    error: 5,
                    message: Some("CallFiltered")
                }),
            );

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance
            );

            assert_ok!(TransactionPause::unpause_transaction(
                Origin::root(),
                b"Balances".to_vec(),
                b"transfer".to_vec()
            ));

            assert_ok!(call.dispatch(Origin::signed(Accounts::ALICE.account())));

            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                initial_balance - to_transfer
            );
            let events = System::events();
            let mut events_iter = events.iter();
            assert_eq!(
                events_iter.next().unwrap(),
                &EventRecord {
                    phase: Phase::Initialization,
                    event: Event::TransactionPause(PauseEvent::TransactionUnpaused(
                        b"Balances".to_vec(),
                        b"transfer".to_vec()
                    )),
                    topics: [].to_vec()
                }
            );
        });
}
