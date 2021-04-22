use core::convert::TryFrom;
use core::convert::TryInto;
use move_vm::data::BalanceAccess;
use move_vm_types::natives::balance::Balance as VmBalance;

// #[cfg(feature = "no-vm-static")]
pub type BalancesAdapter<T> = MoveBalancesAdapter<T>;
// #[cfg(not(feature = "no-vm-static"))]
// pub type BalancesAdapter<T> = DummyBalancesAdapter;

use crate::addr::address_to_account;
use frame_support::traits::Currency;
use frame_support::traits::WithdrawReasons;
use frame_support::traits::ExistenceRequirement;

type BalanceOf<T> = <T as balances::Config>::Balance;

pub struct MoveBalancesAdapter<T>(core::marker::PhantomData<T>);

impl<T: balances::Config> Default for MoveBalancesAdapter<T> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T: balances::Config> MoveBalancesAdapter<T> {
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T: balances::Config> BalanceAccess for MoveBalancesAdapter<T>
where
    <T as balances::Config>::Balance: TryFrom<VmBalance>,
    <T as balances::Config>::Balance: TryInto<VmBalance>,
{
    fn get_balance(
        &self,
        address: &move_core_types::account_address::AccountAddress,
        _ticker: &str,
    ) -> Option<VmBalance> {
        let address = address_to_account::<T::AccountId>(&address).unwrap();
        <balances::Module<T> as Currency<T::AccountId>>::total_balance(&address)
            .try_into()
            .map_err(|_err| error!("Convert native balance to VM balance type."))
            .ok()
    }

    fn deposit(
        &self,
        address: &move_core_types::account_address::AccountAddress,
        _ticker: &str,
        amount: VmBalance,
    ) {
        let _ = address_to_account::<T::AccountId>(&address)
            .map_err(|_err| error!("Can't convert address from Move to Substrate."))
            .and_then(|address| {
                amount
                    .try_into()
                    .map_err(|_err| error!("Can't convert VM balance to native balance type."))
                    .and_then(|amount: BalanceOf<T>| {
                        <balances::Module<T> as Currency<T::AccountId>>::deposit_into_existing(
                            &address, amount,
                        )
                        .map_err(|_err| error!("Can't deposit into existing native balance."))
                    })
            })
            // TODO: return result
            .ok();
    }

    fn withdraw(
        &self,
        address: &move_core_types::account_address::AccountAddress,
        _ticker: &str,
        amount: VmBalance,
    ) {
        let address = address_to_account::<T::AccountId>(&address).unwrap();
        let _ = amount
            .try_into()
            .map_err(|_err| error!("Can't convert VM balance to native balance type."))
            .and_then(|amount: BalanceOf<T>| {
                <balances::Module<T> as Currency<T::AccountId>>::withdraw(
                    &address,
                    amount,
                    WithdrawReasons::RESERVE,
                    ExistenceRequirement::AllowDeath,
                )
                .map_err(|_err| error!("Can't withdraw native balance."))
            })
            // TODO: return result
            .ok();
    }
}

#[cfg(not(feature = "no-vm-static"))]
pub mod boxed {
    use sp_std::prelude::*;
    use move_core_types::account_address::AccountAddress;
    use move_vm_types::natives::balance::Balance as VmBalance;
    use move_vm::data::BalanceAccess;

    pub type BalancesAdapter = MoveBalancesBoxedAdapter;

    /// Vm storage boxed adapter for native storage
    pub struct MoveBalancesBoxedAdapter {
        f_get: Box<dyn Fn(&AccountAddress, &str) -> Option<VmBalance>>,
        f_deposit: Box<dyn Fn(&AccountAddress, &str, VmBalance)>,
        f_withdraw: Box<dyn Fn(&AccountAddress, &str, VmBalance)>,
    }

    impl<T: balances::Config> From<super::MoveBalancesAdapter<T>> for MoveBalancesBoxedAdapter {
        fn from(_balances: super::MoveBalancesAdapter<T>) -> Self {
            Self {
                f_get: Box::new(|address, ticker| {
                    let adapter = super::MoveBalancesAdapter::<T>::new();
                    adapter.get_balance(address, ticker)
                }),
                f_deposit: Box::new(|address, ticker, amount| {
                    let adapter = super::MoveBalancesAdapter::<T>::new();
                    adapter.deposit(address, ticker, amount)
                }),
                f_withdraw: Box::new(|address, ticker, amount| {
                    let adapter = super::MoveBalancesAdapter::<T>::new();
                    adapter.withdraw(address, ticker, amount)
                }),
            }
        }
    }

    impl<T: balances::Config + 'static> From<&'static super::MoveBalancesAdapter<T>>
        for MoveBalancesBoxedAdapter
    {
        fn from(balances: &'static super::MoveBalancesAdapter<T>) -> Self {
            Self {
                f_get: Box::new(move |addr, id| balances.get_balance(addr, id)),
                f_deposit: Box::new(move |addr, id, val| balances.deposit(addr, id, val)),
                f_withdraw: Box::new(move |addr, id, val| balances.withdraw(addr, id, val)),
            }
        }
    }

    impl BalanceAccess for MoveBalancesBoxedAdapter {
        fn get_balance(&self, address: &AccountAddress, ticker: &str) -> Option<VmBalance> {
            trace!("balances::get {} for {}", ticker, address);
            (self.f_get)(address, ticker)
        }

        fn deposit(&self, address: &AccountAddress, ticker: &str, amount: VmBalance) {
            trace!(
                "balances::create (deposit) {} {} for {}",
                ticker,
                amount,
                address
            );
            (self.f_deposit)(address, ticker, amount)
        }

        fn withdraw(&self, address: &AccountAddress, ticker: &str, amount: VmBalance) {
            trace!(
                "balances::destroy (withdraw) {} {} for {}",
                ticker,
                amount,
                address
            );
            (self.f_withdraw)(address, ticker, amount)
        }
    }
}
