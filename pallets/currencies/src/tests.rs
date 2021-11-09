// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Unit tests for the currencies module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
    alice, bob, eva, AccountId, AdaptedBasicCurrency, CouncilAccount, Currencies, DustAccount,
    Event, ExtBuilder, NativeCurrency, Origin, PalletBalances, Runtime, System, Tokens, DOT,
    ID_1, NATIVE_CURRENCY_ID, X_TOKEN_ID,
};
use sp_runtime::traits::BadOrigin;

#[test]
fn multi_lockable_currency_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Currencies::set_lock(ID_1, X_TOKEN_ID, &alice(), 50));
            assert_eq!(Tokens::locks(&alice(), X_TOKEN_ID).len(), 1);
            assert_ok!(Currencies::set_lock(ID_1, NATIVE_CURRENCY_ID, &alice(), 50));
            assert_eq!(PalletBalances::locks(&alice()).len(), 1);
        });
}

#[test]
fn multi_reservable_currency_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(Currencies::total_issuance(NATIVE_CURRENCY_ID), 200);
            assert_eq!(Currencies::total_issuance(X_TOKEN_ID), 200);
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 100);
            assert_eq!(NativeCurrency::free_balance(&alice()), 100);

            assert_ok!(Currencies::reserve(X_TOKEN_ID, &alice(), 30));
            assert_ok!(Currencies::reserve(NATIVE_CURRENCY_ID, &alice(), 40));
            assert_eq!(Currencies::reserved_balance(X_TOKEN_ID, &alice()), 30);
            assert_eq!(
                Currencies::reserved_balance(NATIVE_CURRENCY_ID, &alice()),
                40
            );
        });
}

#[test]
fn native_currency_lockable_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(NativeCurrency::set_lock(ID_1, &alice(), 10));
            assert_eq!(PalletBalances::locks(&alice()).len(), 1);
            assert_ok!(NativeCurrency::remove_lock(ID_1, &alice()));
            assert_eq!(PalletBalances::locks(&alice()).len(), 0);
        });
}

#[test]
fn native_currency_reservable_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(NativeCurrency::reserve(&alice(), 50));
            assert_eq!(NativeCurrency::reserved_balance(&alice()), 50);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_lockable() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(AdaptedBasicCurrency::set_lock(ID_1, &alice(), 10));
            assert_eq!(PalletBalances::locks(&alice()).len(), 1);
            assert_ok!(AdaptedBasicCurrency::remove_lock(ID_1, &alice()));
            assert_eq!(PalletBalances::locks(&alice()).len(), 0);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_reservable() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(AdaptedBasicCurrency::reserve(&alice(), 50));
            assert_eq!(AdaptedBasicCurrency::reserved_balance(&alice()), 50);
        });
}

#[test]
fn multi_currency_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Currencies::transfer(
                Some(alice()).into(),
                bob(),
                X_TOKEN_ID,
                50
            ));
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 50);
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &bob()), 150);
        });
}

#[test]
fn multi_currency_extended_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(
                <Currencies as MultiCurrencyExtended<AccountId>>::update_balance(
                    X_TOKEN_ID,
                    &alice(),
                    50
                )
            );
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 150);
        });
}

#[test]
fn native_currency_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Currencies::transfer_native_currency(
                Some(alice()).into(),
                bob(),
                50
            ));
            assert_eq!(NativeCurrency::free_balance(&alice()), 50);
            assert_eq!(NativeCurrency::free_balance(&bob()), 150);

            assert_ok!(NativeCurrency::transfer(&alice(), &bob(), 10));
            assert_eq!(NativeCurrency::free_balance(&alice()), 40);
            assert_eq!(NativeCurrency::free_balance(&bob()), 160);

            assert_eq!(Currencies::slash(NATIVE_CURRENCY_ID, &alice(), 10), 0);
            assert_eq!(NativeCurrency::free_balance(&alice()), 30);
            assert_eq!(NativeCurrency::total_issuance(), 190);
        });
}

#[test]
fn native_currency_extended_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(NativeCurrency::update_balance(&alice(), 10));
            assert_eq!(NativeCurrency::free_balance(&alice()), 110);

            assert_ok!(
                <Currencies as MultiCurrencyExtended<AccountId>>::update_balance(
                    NATIVE_CURRENCY_ID,
                    &alice(),
                    10
                )
            );
            assert_eq!(NativeCurrency::free_balance(&alice()), 120);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_transfer() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(AdaptedBasicCurrency::transfer(&alice(), &bob(), 50));
            assert_eq!(PalletBalances::total_balance(&alice()), 50);
            assert_eq!(PalletBalances::total_balance(&bob()), 150);

            // creation fee
            assert_ok!(AdaptedBasicCurrency::transfer(&alice(), &eva(), 10));
            assert_eq!(PalletBalances::total_balance(&alice()), 40);
            assert_eq!(PalletBalances::total_balance(&eva()), 10);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_deposit() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(AdaptedBasicCurrency::deposit(&eva(), 50));
            assert_eq!(PalletBalances::total_balance(&eva()), 50);
            assert_eq!(PalletBalances::total_issuance(), 250);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_deposit_throw_error_when_actual_deposit_is_not_expected(
) {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(PalletBalances::total_balance(&eva()), 0);
            assert_eq!(PalletBalances::total_issuance(), 200);
            assert_noop!(
                AdaptedBasicCurrency::deposit(&eva(), 1),
                Error::<Runtime>::DepositFailed
            );
            assert_eq!(PalletBalances::total_balance(&eva()), 0);
            assert_eq!(PalletBalances::total_issuance(), 200);
            assert_ok!(AdaptedBasicCurrency::deposit(&eva(), 2));
            assert_eq!(PalletBalances::total_balance(&eva()), 2);
            assert_eq!(PalletBalances::total_issuance(), 202);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_withdraw() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(AdaptedBasicCurrency::withdraw(&alice(), 100));
            assert_eq!(PalletBalances::total_balance(&alice()), 0);
            assert_eq!(PalletBalances::total_issuance(), 100);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_slash() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(AdaptedBasicCurrency::slash(&alice(), 101), 1);
            assert_eq!(PalletBalances::total_balance(&alice()), 0);
            assert_eq!(PalletBalances::total_issuance(), 100);
        });
}

#[test]
fn basic_currency_adapting_pallet_balances_update_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(AdaptedBasicCurrency::update_balance(&alice(), -10));
            assert_eq!(PalletBalances::total_balance(&alice()), 90);
            assert_eq!(PalletBalances::total_issuance(), 190);
        });
}

#[test]
fn update_balance_call_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Currencies::update_balance(
                Origin::root(),
                alice(),
                NATIVE_CURRENCY_ID,
                -10
            ));
            assert_eq!(NativeCurrency::free_balance(&alice()), 90);
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 100);
            assert_ok!(Currencies::update_balance(
                Origin::root(),
                alice(),
                X_TOKEN_ID,
                10
            ));
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 110);
        });
}

#[test]
fn update_balance_call_fails_if_not_root_origin() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Currencies::update_balance(Some(alice()).into(), alice(), X_TOKEN_ID, 100),
            BadOrigin
        );
    });
}

#[test]
fn call_event_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Currencies::transfer(
                Some(alice()).into(),
                bob(),
                X_TOKEN_ID,
                50
            ));
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 50);
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &bob()), 150);
            System::assert_last_event(Event::Currencies(crate::Event::Transferred(
                X_TOKEN_ID,
                alice(),
                bob(),
                50,
            )));

            assert_ok!(<Currencies as MultiCurrency<AccountId>>::transfer(
                X_TOKEN_ID,
                &alice(),
                &bob(),
                10
            ));
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 40);
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &bob()), 160);
            System::assert_last_event(Event::Currencies(crate::Event::Transferred(
                X_TOKEN_ID,
                alice(),
                bob(),
                10,
            )));

            assert_ok!(<Currencies as MultiCurrency<AccountId>>::deposit(
                X_TOKEN_ID,
                &alice(),
                100
            ));
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 140);
            System::assert_last_event(Event::Currencies(crate::Event::Deposited(
                X_TOKEN_ID,
                alice(),
                100,
            )));

            assert_ok!(<Currencies as MultiCurrency<AccountId>>::withdraw(
                X_TOKEN_ID,
                &alice(),
                20
            ));
            assert_eq!(Currencies::free_balance(X_TOKEN_ID, &alice()), 120);
            System::assert_last_event(Event::Currencies(crate::Event::Withdrawn(
                X_TOKEN_ID,
                alice(),
                20,
            )));
        });
}

#[test]
fn sweep_dust_tokens_works() {
    ExtBuilder::default().build().execute_with(|| {
        orml_tokens::Accounts::<Runtime>::insert(
            bob(),
            DOT,
            orml_tokens::AccountData {
                free: 1,
                frozen: 0,
                reserved: 0,
            },
        );
        orml_tokens::Accounts::<Runtime>::insert(
            eva(),
            DOT,
            orml_tokens::AccountData {
                free: 2,
                frozen: 0,
                reserved: 0,
            },
        );
        orml_tokens::Accounts::<Runtime>::insert(
            alice(),
            DOT,
            orml_tokens::AccountData {
                free: 0,
                frozen: 1,
                reserved: 0,
            },
        );
        orml_tokens::Accounts::<Runtime>::insert(
            DustAccount::get(),
            DOT,
            orml_tokens::AccountData {
                free: 100,
                frozen: 0,
                reserved: 0,
            },
        );
        orml_tokens::TotalIssuance::<Runtime>::insert(DOT, 104);

        let accounts = vec![bob(), eva(), alice()];

        assert_noop!(
            Currencies::sweep_dust(Origin::signed(bob()), DOT, accounts.clone()),
            DispatchError::BadOrigin
        );

        assert_ok!(Currencies::sweep_dust(
            Origin::signed(CouncilAccount::get()),
            DOT,
            accounts.clone()
        ));
        System::assert_last_event(Event::Currencies(crate::Event::DustSwept(DOT, bob(), 1)));

        // bob's account is gone
        assert_eq!(
            orml_tokens::Accounts::<Runtime>::contains_key(bob(), DOT),
            false
        );
        assert_eq!(Currencies::free_balance(DOT, &bob()), 0);

        // eva's account remains, not below ED
        assert_eq!(Currencies::free_balance(DOT, &eva()), 2);

        // Dust transferred to dust receiver
        assert_eq!(Currencies::free_balance(DOT, &DustAccount::get()), 101);
        // Total issuance remains the same
        assert_eq!(Currencies::total_issuance(DOT), 104);
    });
}

#[test]
fn sweep_dust_native_currency_works() {
    use frame_support::traits::StoredMap;
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
            &bob(),
            pallet_balances::AccountData {
                free: 1,
                reserved: 0,
                misc_frozen: 0,
                fee_frozen: 0,
            },
        ));
        assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
            &eva(),
            pallet_balances::AccountData {
                free: 2,
                reserved: 0,
                misc_frozen: 0,
                fee_frozen: 0,
            },
        ));
        assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
            &alice(),
            pallet_balances::AccountData {
                free: 0,
                reserved: 0,
                misc_frozen: 2,
                fee_frozen: 2,
            },
        ));
        assert_ok!(<Runtime as pallet_balances::Config>::AccountStore::insert(
            &DustAccount::get(),
            pallet_balances::AccountData {
                free: 100,
                reserved: 0,
                misc_frozen: 0,
                fee_frozen: 0,
            },
        ));
        pallet_balances::TotalIssuance::<Runtime>::put(104);

        assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &bob()), 1);
        assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &eva()), 2);
        assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &alice()), 0);
        assert_eq!(
            Currencies::free_balance(NATIVE_CURRENCY_ID, &DustAccount::get()),
            100
        );

        let accounts = vec![bob(), eva(), alice()];

        assert_noop!(
            Currencies::sweep_dust(Origin::signed(bob()), NATIVE_CURRENCY_ID, accounts.clone()),
            DispatchError::BadOrigin
        );

        assert_ok!(Currencies::sweep_dust(
            Origin::signed(CouncilAccount::get()),
            NATIVE_CURRENCY_ID,
            accounts.clone()
        ));
        System::assert_last_event(Event::Currencies(crate::Event::DustSwept(
            NATIVE_CURRENCY_ID,
            bob(),
            1,
        )));

        // bob's account is gone
        assert_eq!(System::account_exists(&bob()), false);
        assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &bob()), 0);

        // eva's account remains, not below ED
        assert_eq!(Currencies::free_balance(NATIVE_CURRENCY_ID, &eva()), 2);

        // Dust transferred to dust receiver
        assert_eq!(
            Currencies::free_balance(NATIVE_CURRENCY_ID, &DustAccount::get()),
            101
        );
        // Total issuance remains the same
        assert_eq!(Currencies::total_issuance(NATIVE_CURRENCY_ID), 104);
    });
}
