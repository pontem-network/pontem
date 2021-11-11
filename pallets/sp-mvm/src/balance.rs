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
use frame_support::pallet_prelude::MaybeSerializeDeserialize;
use frame_support::dispatch::fmt::Debug;
use parity_scale_codec::{FullCodec, Decode};
use sp_std::cmp::PartialEq;
use move_vm::io::balance::CurrencyInfo;
use sp_std::{vec::Vec, prelude::*, default::Default};

/// Balance Adapter struct.
pub struct BalancesAdapter<AccountId, Currencies, CurrencyId>(core::marker::PhantomData<(AccountId, Currencies, CurrencyId)>);

/// Default Balance Adapter.
impl<AccountId, Currencies, CurrencyId> Default for BalancesAdapter<AccountId, Currencies, CurrencyId> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<AccountId, Currencies, CurrencyId> BalancesAdapter<AccountId, Currencies, CurrencyId> {
    /// Create new instance of Balance Adapter.
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

/// Implement balance BalanceAccess trait for Balances Adapter.
///
/// It's a trait required to Move VM and allows for poxy balances between Substrate and VM.
impl<
    AccountId: Decode + Sized,
    Currencies: orml_traits::MultiCurrency<AccountId, CurrencyId = CurrencyId>, 
    CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug + TryFrom<Vec<u8>> + Default
    > BalanceAccess 
    for BalancesAdapter<AccountId, Currencies, CurrencyId>
where
    Currencies::Balance: TryFrom<VmBalance>
{
    /// Query native coin balance.
    ///
    /// We check if ticker supported, and if yes return account balance to Move VM.
    fn get_balance(
        &self,
        address: &move_core_types::account_address::AccountAddress,
        ticker: &[u8],
    ) -> Option<VmBalance> {
        let currency_id = CurrencyId::try_from(ticker.to_vec());

        match currency_id {
            Ok(id) => {
                address_to_account::<AccountId>(address)
                .map_err(|_| error!("Can't convert address from Move to Substrate."))
                .and_then(|address| {
                    Currencies::free_balance(id, &address)
                        .try_into()
                        .map_err(|_err| error!("Convert native balance to VM balance type."))
                })
                .ok()
            },
            Err(_) => {
                trace!("native balance ticker not supported");
                return None;
            }
        }

        //trace!(
        //    "native balance requested for address: {} (ticker: {})",
        //    address,
        //    currencyId
        //);
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
        let currency_id = CurrencyId::try_from(ticker.to_vec());

        match currency_id {
            Ok(id) => {
                //trace!("withdraw resource {} requested, amount: {}", currencyId, amount);
                address_to_account::<AccountId>(address)
                    .map_err(|_err| error!("Can't convert address from Move to Substrate."))
                    .and_then(|address| {
                        amount
                            .try_into()
                            .map_err(|_err| error!("Can't convert VM balance to native balance type."))
                            .map(|amount: Currencies::Balance| {
                                Currencies::deposit(id, &address, amount)
                            })
                    })
                    .ok();
                //trace!("native balance deposit imbalance: {:?}", imbalance);
            },
            Err(e) => trace!("add: error getting currency id")
        }

        
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
        let currency_id = CurrencyId::try_from(ticker.to_vec());

        match currency_id {
            Ok(id) => {
                //trace!("deposit resource {} requested, amount: {}", currencyId, amount);
                address_to_account::<AccountId>(address)
                    .map_err(|_| error!("Can't convert address from Move to Substrate."))
                    .and_then(|address| {
                        amount
                            .try_into()
                            .map_err(|_err| error!("Can't convert VM balance to native balance type."))
                            .and_then(|amount: Currencies::Balance| {
                                Currencies::withdraw(id, &address, amount).map_err(|_err| error!("Can't withdraw native balance."))
                            })
                    })
                    .ok();
                //trace!("native balance withdraw imbalance: {:?}", imbalance);
            },
            Err(e) => trace!("sub: error getting currency id"),
        }
    }

    // As we have only one currency now, calling PONT, we ignore paths.
    // TODO: support tickets instead of paths.
    fn get_currency_info(
        &self,
        _path: &move_vm::io::traits::CurrencyAccessPath,
    ) -> Option<CurrencyInfo> {
        match Currencies::total_issuance(CurrencyId::default()).try_into() {
            Ok(total_value) => Some(CurrencyInfo { total_value }),
            Err(_) => None,
        }
    }
}

#[cfg(not(feature = "no-vm-static"))]
pub mod boxed {
    use move_vm::io::{
        traits::{Balance as VmBalance, BalanceAccess},
    };
    use sp_std::prelude::*;
    use sp_std::convert::TryFrom;
    use frame_support::pallet_prelude::MaybeSerializeDeserialize;
    use frame_support::dispatch::fmt::Debug;
    use parity_scale_codec::{FullCodec, Decode};
    use move_core_types::account_address::AccountAddress;

    pub type BalancesAdapter = BalancesBoxedAdapter;

    /// Vm storage boxed adapter for native storage
    pub struct BalancesBoxedAdapter {
        f_get: Box<dyn Fn(&AccountAddress, &[u8]) -> Option<VmBalance>>,
        f_deposit: Box<dyn Fn(&AccountAddress, &[u8], VmBalance)>,
        f_withdraw: Box<dyn Fn(&AccountAddress, &[u8], VmBalance)>,
    }

    impl<
            AccountId: Decode + Sized + 'static,
            Currencies: orml_traits::MultiCurrency<AccountId, CurrencyId = CurrencyId> + 'static, 
            CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug + TryFrom<Vec<u8>> + Default + 'static
        > 
        From<super::BalancesAdapter<AccountId, Currencies, CurrencyId>> 
        for BalancesBoxedAdapter 
        {
            fn from(adapter: super::BalancesAdapter<AccountId, Currencies, CurrencyId>) -> Self {
                Self {
                    f_get: Box::new(move |address, ticker| adapter.get_balance(address, ticker)),
                    f_deposit: Box::new(|address, ticker, amount| {
                        let adapter = super::BalancesAdapter::<AccountId, Currencies, CurrencyId>::new();
                        adapter.add(address, ticker, amount)
                    }),
                    f_withdraw: Box::new(|address, ticker, amount| {
                        let adapter = super::BalancesAdapter::<AccountId, Currencies, CurrencyId>::new();
                        adapter.sub(address, ticker, amount)
                    }),
                }
            }
    }

    impl<
            AccountId: Decode + Sized + 'static,
            Currencies: orml_traits::MultiCurrency<AccountId, CurrencyId = CurrencyId> + 'static, 
            CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug + TryFrom<Vec<u8>> + Default + 'static
        > 
        From<&'static super::BalancesAdapter<AccountId, Currencies, CurrencyId>>
        for BalancesBoxedAdapter
    {
        fn from(balances: &'static super::BalancesAdapter<AccountId, Currencies, CurrencyId>) -> Self {
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
