// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! The current file takes care of the connection of native coins inside Move VM.
//!
//! There is native chain coin called PONT, we need to have a coin inside the Move VM and allows developers to get access to PONT balances: transfer it, get balance etc.
//! PONT is similar to ETH in the case of EVM.
//! To see how to transfer PONT coin using Move VM modules/scripts, read tutorial - https://docs.pontem.network/02.-getting-started/first_transaction#transfer-coins-via-script
//! In the current file we implement a Balance Adapter that catches PONT balances changes, and freeze balance or add balance to account in case Move VM access PONT balance resource.
//! We are utilizing a balance pallet here.

use core::convert::TryFrom;
use core::convert::TryInto;
use move_vm::io::traits::{Balance as VmBalance, BalanceAccess};

use crate::addr::address_to_account;
use frame_support::traits::Currency;
use frame_support::traits::Imbalance;
use frame_support::traits::WithdrawReasons;
use frame_support::traits::ExistenceRequirement;
use frame_support::traits::fungible::Inspect;
use move_vm::io::balance::CurrencyInfo;

/// Balance type.
type BalanceOf<T> = <T as balances::Config>::Balance;

/// PONT ticker.
pub const PONT: Ticker = Ticker::new("PONT");

/// Supported tickers.
pub static TICKERS: &[Ticker] = &[PONT];

/// Check if ticker supports as native balance.
pub fn is_ticker_supported(ticker: Ticker) -> bool {
    TICKERS.contains(&ticker)
}

/// Balance Adapter struct.
pub struct BalancesAdapter<T>(core::marker::PhantomData<T>);

/// Default Balance Adapter.
impl<T: balances::Config> Default for BalancesAdapter<T> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T: balances::Config> BalancesAdapter<T> {
    /// Create new instance of Balance Adapter.
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
/// Ticker struct.
pub struct Ticker<'a>(&'a [u8]);

/// Display trait impl for ticker struct.
impl<'a> core::fmt::Display for Ticker<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(self.0).expect("Could not read as utf-8"))
            .expect("Could not write into formatter");
        Ok(())
    }
}

impl<'a> From<&'a [u8]> for Ticker<'a> {
    /// Convert ticker from bytes slice.
    fn from(f: &[u8]) -> Ticker<'_> {
        Ticker(f)
    }
}

impl<'a> From<&'a str> for Ticker<'a> {
    /// Convert ticker from &str.
    fn from(f: &str) -> Ticker<'_> {
        Ticker(f.as_bytes())
    }
}

impl Ticker<'_> {
    /// New ticker from &str representation.
    pub const fn new(ticker: &str) -> Ticker {
        Ticker(ticker.as_bytes())
    }
}

/// Implement balance BalanceAccess trait for Balances Adapter.
///
/// It's a trait required to Move VM and allows for poxy balances between Substrate and VM.
impl<T: balances::Config> BalanceAccess for BalancesAdapter<T>
where
    <T as balances::Config>::Balance: TryFrom<VmBalance>,
{
    /// Query native coin balance.
    ///
    /// We check if ticker supported, and if yes return account balance to Move VM.
    fn get_balance(
        &self,
        address: &move_core_types::account_address::AccountAddress,
        ticker: &[u8],
    ) -> Option<VmBalance> {
        let ticker = Ticker(ticker);

        if !is_ticker_supported(ticker) {
            trace!("native balance ticker '{}' not supported", ticker);
            return None;
        }

        trace!(
            "native balance requested for address: {} (ticker: {})",
            address,
            ticker
        );
        address_to_account::<T::AccountId>(address)
            .map_err(|_| error!("Can't convert address from Move to Substrate."))
            .and_then(|address| {
                <balances::Pallet<T>>::reducible_balance(&address, false)
                    .try_into()
                    .map_err(|_err| error!("Convert native balance to VM balance type."))
            })
            .ok()
    }

    /// Add native coins to account.
    ///
    /// We increase native coin balance of the account if transfer of balance happens inside the VM.
    fn add(
        &self,
        address: &move_core_types::account_address::AccountAddress,
        ticker: &[u8],
        amount: VmBalance,
    ) {
        let ticker = Ticker(ticker);

        if !is_ticker_supported(ticker) {
            return trace!("native balance ticker '{}' not supported", ticker);
        }

        trace!("withdraw resource {} requested, amount: {}", ticker, amount);
        let imbalance = address_to_account::<T::AccountId>(address)
            .map_err(|_err| error!("Can't convert address from Move to Substrate."))
            .and_then(|address| {
                amount
                    .try_into()
                    .map_err(|_err| error!("Can't convert VM balance to native balance type."))
                    .map(|amount: BalanceOf<T>| {
                        <balances::Pallet<T> as Currency<T::AccountId>>::deposit_creating(
                            &address, amount,
                        )
                    })
            })
            .map(|imbalance| imbalance.peek())
            .ok();
        trace!("native balance deposit imbalance: {:?}", imbalance);
    }

    /// Reduce native coin balance of account.
    ///
    /// We reduce native coin balance if transfer of balance happens inside a VM.
    fn sub(
        &self,
        address: &move_core_types::account_address::AccountAddress,
        ticker: &[u8],
        amount: VmBalance,
    ) {
        let ticker = Ticker(ticker);

        if !is_ticker_supported(ticker) {
            return trace!("native balance ticker '{}' not supported", ticker);
        }

        trace!("deposit resource {} requested, amount: {}", ticker, amount);
        let imbalance = address_to_account::<T::AccountId>(address)
            .map_err(|_| error!("Can't convert address from Move to Substrate."))
            .and_then(|address| {
                amount
                    .try_into()
                    .map_err(|_err| error!("Can't convert VM balance to native balance type."))
                    .and_then(|amount: BalanceOf<T>| {
                        <balances::Pallet<T> as Currency<T::AccountId>>::withdraw(
                            &address,
                            amount,
                            WithdrawReasons::RESERVE | WithdrawReasons::TRANSFER,
                            ExistenceRequirement::AllowDeath,
                        )
                        .map_err(|_err| error!("Can't withdraw native balance."))
                    })
                    .map(|imbalance| imbalance.peek())
            })
            .ok();
        trace!("native balance withdraw imbalance: {:?}", imbalance);
    }

    // As we have only one currency now, calling PONT, we ignore paths.
    // TODO: support paths.
    fn get_currency_info(
        &self,
        _path: &move_vm::io::traits::CurrencyAccessPath,
    ) -> Option<CurrencyInfo> {
        match <balances::Pallet<T> as Currency<T::AccountId>>::total_issuance().try_into() {
            Ok(total_value) => Some(CurrencyInfo { total_value }),
            Err(_) => None,
        }
    }
}

#[cfg(not(feature = "no-vm-static"))]
pub mod boxed {
    use move_vm::io::{
        //balance::CurrencyInfo,
        traits::{Balance as VmBalance, BalanceAccess},
    };
    use sp_std::prelude::*;
    use move_core_types::account_address::AccountAddress;

    pub type BalancesAdapter = BalancesBoxedAdapter;

    /// Vm storage boxed adapter for native storage
    pub struct BalancesBoxedAdapter {
        f_get: Box<dyn Fn(&AccountAddress, &[u8]) -> Option<VmBalance>>,
        f_deposit: Box<dyn Fn(&AccountAddress, &[u8], VmBalance)>,
        f_withdraw: Box<dyn Fn(&AccountAddress, &[u8], VmBalance)>,
    }

    impl<T: balances::Config> From<super::BalancesAdapter<T>> for BalancesBoxedAdapter {
        fn from(adapter: super::BalancesAdapter<T>) -> Self {
            Self {
                f_get: Box::new(move |address, ticker| adapter.get_balance(address, ticker)),
                f_deposit: Box::new(|address, ticker, amount| {
                    let adapter = super::BalancesAdapter::<T>::new();
                    adapter.add(address, ticker, amount)
                }),
                f_withdraw: Box::new(|address, ticker, amount| {
                    let adapter = super::BalancesAdapter::<T>::new();
                    adapter.sub(address, ticker, amount)
                }),
            }
        }
    }

    impl<T: balances::Config + 'static> From<&'static super::BalancesAdapter<T>>
        for BalancesBoxedAdapter
    {
        fn from(balances: &'static super::BalancesAdapter<T>) -> Self {
            Self {
                f_get: Box::new(move |addr, id| balances.get_balance(addr, id)),
                f_deposit: Box::new(move |addr, id, val| balances.add(addr, id, val)),
                f_withdraw: Box::new(move |addr, id, val| balances.sub(addr, id, val)),
            }
        }
    }

    impl BalanceAccess for BalancesBoxedAdapter {
        fn get_balance(&self, address: &AccountAddress, ticker: &[u8]) -> Option<VmBalance> {
            (self.f_get)(address, ticker)
        }

        fn add(&self, address: &AccountAddress, ticker: &[u8], amount: VmBalance) {
            (self.f_deposit)(address, ticker, amount)
        }

        fn sub(&self, address: &AccountAddress, ticker: &[u8], amount: VmBalance) {
            (self.f_withdraw)(address, ticker, amount)
        }

        fn get_currency_info(
            &self,
            _path: &move_vm::io::traits::CurrencyAccessPath,
        ) -> Option<move_vm::io::balance::CurrencyInfo> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PONT;

    #[test]
    fn is_ticker_supported() {
        assert!(!super::is_ticker_supported(super::Ticker::new(
            "NOT_SUPPORTED"
        )));
        assert!(super::is_ticker_supported(PONT));
        assert!(super::is_ticker_supported(super::Ticker::new("PONT")));
        assert!(super::is_ticker_supported("PONT".into()));
    }
}
