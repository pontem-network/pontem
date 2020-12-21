#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]

#[macro_use]
extern crate log;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_support::dispatch::DispatchResult;

use codec::{Encode, Decode};
use move_vm_types::values::values_impl::Value;
use move_core_types::language_storage::TypeTag;
use move_vm::types::Gas;
use move_vm::types::ScriptTx;
use move_vm::data::{State, Storage};
use sp_std::prelude::*;
pub use move_vm::*;
use move_vm::dvm::Dvm;
use move_core_types::account_address::AccountAddress;
use sp_runtime::print;
use frame_support::debug;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Create or get cached VM
// fn get_vm() -> Dvm<StorageImpl<VMStorage>> {
    fn get_vm() -> impl move_vm::Vm {
    let store: StorageImpl<VMStorage> = Default::default();
    move_vm::dvm::Dvm::new(store)
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
     // A unique name is used to ensure that the pallet's storage items are isolated.
     // This name may be updated, but each pallet in the runtime must use a unique name.
     // ---------------------------------vvvvvvvvvvvvvv
     trait Store for Module<T: Trait> as Mvm {
         // Learn more about declaring storage items:
         // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items

         /// Storage for move- write-sets contains code & resources
         VMStorage get(fn vmstorage): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
     }
}

struct StorageImpl<T>(core::marker::PhantomData<T>);

impl<T> Default for StorageImpl<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Storage for StorageImpl<VMStorage> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        VMStorage::get(key)
    }

    fn insert(&self, key: &[u8], value: &[u8]) {
        VMStorage::insert(key, value)
    }

    fn remove(&self, key: &[u8]) {
        VMStorage::remove(key)
    }
}

pub trait AccountIdAsBytes<AccountId, T: Sized> {
    fn account_as_bytes(acc: &AccountId) -> T;
}

impl<T> AccountIdAsBytes<T::AccountId, Vec<u8>> for T
where
    T: frame_system::Trait,
    T::AccountId: Encode,
{
    fn account_as_bytes(acc: &T::AccountId) -> Vec<u8> {
        acc.encode()
    }
}

impl<T> AccountIdAsBytes<T::AccountId, [u8; AccountAddress::LENGTH]> for T
where
    T: frame_system::Trait,
    T::AccountId: Encode,
{
    fn account_as_bytes(acc: &T::AccountId) -> [u8; AccountAddress::LENGTH] {
        use core::convert::TryInto;
        const LENGTH: usize = AccountAddress::LENGTH;

        let mut bytes = acc.encode();
        // add zero-padding
        while bytes.len() < LENGTH {
            bytes.push(0);
        }
        // convert to array
        let boxed_slice = bytes.into_boxed_slice();
        let boxed_array: Box<[u8; LENGTH]> = match boxed_slice.try_into() {
            Ok(ba) => ba,
            Err(o) => panic!("Expected a Vec of length {} but it was {}", LENGTH, o.len()),
        };
        *boxed_array
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        // Event documentation should end with an array that provides descriptive names for event parameters.
        /// [unwrapped_res, who]
        ResourceMoved(u128, AccountId),
    }
);

// Errors inform users that something went wrong.
decl_error! {
     pub enum Error for Module<T: Trait> {
          /// Error names should be descriptive.
          NoneValue,
          /// Errors should have helpful documentation associated with them.
          StorageOverflow,

          ScriptTxValidationError,
          ScriptTxExecutionError,
     }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
          // Errors must be initialized if they are used by the pallet.
          type Error = Error<T>;

          // Events must be initialized if they are used by the pallet.
          fn deposit_event() = default;

          /// An example dispatchable that takes a singles value as a parameter, writes the value to
          /// storage and emits an event. This function must be dispatched by a signed extrinsic.
          #[weight = 10_000 + T::DbWeight::get().writes(1)]
          // TODO: return DispatchResultWithPostInfo with spend gas by vm
          pub fn execute(origin, script_code: Vec<u8>) -> dispatch::DispatchResult {
              info!("Executing `execute`");

              // Check that the extrinsic was signed and get the signer.
              // This function will return an error if the extrinsic is not signed.
              // https://substrate.dev/docs/en/knowledgebase/runtime/origin
              let who = ensure_signed(origin)?;
              info!("`ensure_signed` ok");

              let vm = get_vm();

              // TODO: gas-table & min-max values shoud be in genesis/config
              let max_gas_amount = (u64::MAX / 1000) - 42;
              // TODO: get native value
              let gas_unit_price = 1;
              let gas = Gas::new(max_gas_amount, gas_unit_price).unwrap();

              let tx = {
                  let code: Vec<u8> = script_code;
                  let args: Vec<Value> = Default::default();
                  let type_args: Vec<TypeTag> = Default::default();

                  let sender = T::account_as_bytes(&who);
                  {
                      debug::info!("converted sender: {:?}", sender);
                      #[cfg(feature = "std")]
                      eprintln!("converted sender: {:?}", sender);
                  }

                  let senders: Vec<AccountAddress> = vec![
                      AccountAddress::new(sender),
                  ];

                  ScriptTx::new(code, args, type_args, senders).map_err(|err|{
                      // Error::<T>::ScriptTxValidationError(err.to_string())
                      Error::<T>::ScriptTxValidationError
                  })?
              };

              let res = vm.execute_script(gas, tx);
              #[cfg(feature = "std")]
              eprintln!("result: {:?}", res);

              {
                  debug::info!("execution result: {:?}", res);
                  #[cfg(feature = "std")]
                  eprintln!("execution result: {:?}", res);
              }

              res.map_err(|err|{
                // TODO: unwrap error
                Error::<T>::ScriptTxExecutionError
              })?;

              // TODO: update storage:
              // VMStorage::insert(...);

              // Emit an event:
              // Self::deposit_event(RawEvent::ResourceMoved(unwrapped_res, who));

              // Return a successful DispatchResult
              Ok(())
          }

          // fn on_initialize(n: T::BlockNumber,) -> Weight { if n.into() == 42 { panic!("on_initialize") } 7 }
          // fn on_finalize(n: T::BlockNumber,) { if n.into() == 42 { panic!("on_finalize") } }
          // fn on_runtime_upgrade() -> Weight { 10 }
          // fn offchain_worker() {}
     }
}
