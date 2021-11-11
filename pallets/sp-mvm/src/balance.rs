// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! The current file takes care of the connection of native coins inside Move VM.
//!
//! Multicurrency implementation used to work with multiplay balances.
//! PalletId using to deposit/withdraw tokens to/from current pallet, it solves issue with total issuance.
//!
//! BalancesAdapter methods, all methods implements ticker support:
//!     * get_balance - get current balance of account.
//!     * add - add tokens to account.
//!     * sub - reduce account balance on amount.

use core::convert::TryFrom;
use core::convert::TryInto;
use move_vm::io::traits::{Balance as VmBalance, BalanceAccess};

use crate::addr::address_to_account;
use frame_support::pallet_prelude::MaybeSerializeDeserialize;
use frame_support::dispatch::fmt::Debug;
use frame_support::PalletId;
use parity_scale_codec::{FullCodec, Decode, Encode};
use sp_std::cmp::PartialEq;
use move_vm::io::balance::CurrencyInfo;
use sp_std::{vec::Vec, prelude::*, default::Default};
use sp_runtime::traits::AccountIdConversion;

#[derive(PartialEq, Eq, Clone, Copy)]
/// Ticker struct.
pub struct PrintedTicker<'a>(&'a [u8]);

/// Display trait impl for printed ticker struct.
impl<'a> core::fmt::Display for PrintedTicker<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(self.0).expect("Could not read as utf-8"))
            .expect("Could not write into formatter");
        Ok(())
    }
}

/// Balance Adapter struct.
pub struct BalancesAdapter<AccountId, Currencies, CurrencyId> {
    pallet_id: PalletId,
    _phantom: core::marker::PhantomData<(AccountId, Currencies, CurrencyId)>,
}

impl<AccountId: Encode + Decode + Default, Currencies, CurrencyId>
    BalancesAdapter<AccountId, Currencies, CurrencyId>
{
    /// Create new instance of Balance Adapter.
    pub fn new(pallet_id: PalletId) -> Self {
        Self {
            pallet_id,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Get current pallet id.
    pub fn get_pallet_id(&self) -> PalletId {
        self.pallet_id
    }

    /// Convert pallet id into account.
    pub fn get_pallet_account(&self) -> AccountId {
        self.pallet_id.into_account()
    }
}

/// Implement balance BalanceAccess trait for Balances Adapter.
///
/// It's a trait required to Move VM and allows for poxy balances between Substrate and VM.
/// Using deposit/withdraw to PalletId we are solving total issuance issue.
impl<
        AccountId: Encode + Decode + Default,
        Currencies: orml_traits::MultiCurrency<AccountId, CurrencyId = CurrencyId>,
        CurrencyId: FullCodec
            + Eq
            + PartialEq
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + TryFrom<Vec<u8>>
            + Default,
    > BalanceAccess for BalancesAdapter<AccountId, Currencies, CurrencyId>
where
    Currencies::Balance: TryFrom<VmBalance>,
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
                trace!(
                    "native balance requested for address: {} (ticker: {})",
                    address,
                    PrintedTicker(ticker)
                );

                address_to_account::<AccountId>(address)
                    .map_err(|_| error!("can't convert address from Move to Substrate."))
                    .and_then(|address| {
                        // TODO: replace with reducible_balance.
                        Currencies::free_balance(id, &address)
                            .try_into()
                            .map_err(|_err| {
                                error!("can't convert native balance to VM balance type.")
                            })
                    })
                    .ok()
            }
            Err(_) => {
                trace!(
                    "native balance ticker '{}' not supported",
                    PrintedTicker(ticker)
                );
                return None;
            }
        }
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

        trace!("deposit native balance '{}'", PrintedTicker(ticker));
        match currency_id {
            Ok(id) => {
                address_to_account::<AccountId>(address)
                    .map_err(|_err| error!("Can't convert address from Move to Substrate."))
                    .and_then(|address| {
                        let amount: Currencies::Balance = amount.try_into().map_err(|_err| {
                            error!("Can't convert VM balance to native balance type.")
                        })?;
                        Currencies::withdraw(id, &self.get_pallet_account(), amount)
                            .map_err(|_err| error!("Can't withdraw from pallet"))?;
                        Currencies::deposit(id, &address, amount)
                            .map_err(|_err| error!("Can't deposit native balance."))
                    })
                    .ok();
            }
            Err(_) => trace!(
                "native balance ticker '{}' not supported",
                PrintedTicker(ticker)
            ),
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
                trace!(
                    "withdraw balance {} requested, amount: {}",
                    PrintedTicker(ticker),
                    amount
                );
                address_to_account::<AccountId>(address)
                    .map_err(|_| error!("Can't convert address from Move to Substrate."))
                    .and_then(|address| {
                        let amount: Currencies::Balance = amount.try_into().map_err(|_err| {
                            error!("Can't convert VM balance to native balance type.")
                        })?;
                        Currencies::withdraw(id, &address, amount)
                            .map_err(|_err| error!("Can't deposit native balance."))?;
                        Currencies::deposit(id, &self.get_pallet_account(), amount)
                            .map_err(|_err| error!("Can't withdraw from pallet"))
                    })
                    .ok();
            }
            Err(_) => trace!(
                "native balance ticker '{}' not supported",
                PrintedTicker(ticker)
            ),
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
    use parity_scale_codec::{FullCodec, Decode, Encode};
    use move_core_types::account_address::AccountAddress;
    use frame_support::PalletId;

    pub type BalancesAdapter = BalancesBoxedAdapter;

    /// Vm storage boxed adapter for native storage
    pub struct BalancesBoxedAdapter {
        pallet_id: PalletId,
        f_get: Box<dyn Fn(&AccountAddress, &[u8]) -> Option<VmBalance>>,
        f_deposit: Box<dyn Fn(&PalletId, &AccountAddress, &[u8], VmBalance)>,
        f_withdraw: Box<dyn Fn(&PalletId, &AccountAddress, &[u8], VmBalance)>,
    }

    impl<
            AccountId: Encode + Decode + Sized + Default + 'static,
            Currencies: orml_traits::MultiCurrency<AccountId, CurrencyId = CurrencyId> + 'static,
            CurrencyId: FullCodec
                + Eq
                + PartialEq
                + Copy
                + MaybeSerializeDeserialize
                + Debug
                + TryFrom<Vec<u8>>
                + Default
                + 'static,
        > From<super::BalancesAdapter<AccountId, Currencies, CurrencyId>>
        for BalancesBoxedAdapter
    {
        fn from(adapter: super::BalancesAdapter<AccountId, Currencies, CurrencyId>) -> Self {
            Self {
                pallet_id: adapter.get_pallet_id(),
                f_get: Box::new(move |address, ticker| adapter.get_balance(address, ticker)),
                f_deposit: Box::new(|pallet_id, address, ticker, amount| {
                    let adapter =
                        super::BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(
                            *pallet_id,
                        );
                    adapter.add(address, ticker, amount)
                }),
                f_withdraw: Box::new(|pallet_id, address, ticker, amount| {
                    let adapter =
                        super::BalancesAdapter::<AccountId, Currencies, CurrencyId>::new(
                            *pallet_id,
                        );
                    adapter.sub(address, ticker, amount)
                }),
            }
        }
    }

    impl<
            AccountId: Encode + Decode + Sized + Default + 'static,
            Currencies: orml_traits::MultiCurrency<AccountId, CurrencyId = CurrencyId> + 'static,
            CurrencyId: FullCodec
                + Eq
                + PartialEq
                + Copy
                + MaybeSerializeDeserialize
                + Debug
                + TryFrom<Vec<u8>>
                + Default
                + 'static,
        > From<&'static super::BalancesAdapter<AccountId, Currencies, CurrencyId>>
        for BalancesBoxedAdapter
    {
        fn from(
            balances: &'static super::BalancesAdapter<AccountId, Currencies, CurrencyId>,
        ) -> Self {
            Self {
                pallet_id: balances.get_pallet_id(),
                f_get: Box::new(move |addr, id| balances.get_balance(addr, id)),
                f_deposit: Box::new(move |_, addr, id, val| balances.add(addr, id, val)),
                f_withdraw: Box::new(move |_, addr, id, val| balances.sub(addr, id, val)),
            }
        }
    }

    impl BalanceAccess for BalancesBoxedAdapter {
        fn get_balance(&self, address: &AccountAddress, ticker: &[u8]) -> Option<VmBalance> {
            (self.f_get)(address, ticker)
        }

        fn add(&self, address: &AccountAddress, ticker: &[u8], amount: VmBalance) {
            (self.f_deposit)(&self.pallet_id, address, ticker, amount)
        }

        fn sub(&self, address: &AccountAddress, ticker: &[u8], amount: VmBalance) {
            (self.f_withdraw)(&self.pallet_id, address, ticker, amount)
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
