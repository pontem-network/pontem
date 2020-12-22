#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate log;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use codec::{Encode, Decode};

use move_core_types::value::MoveKind;
use move_core_types::value::MoveKindInfo;
use move_core_types::value::MoveTypeLayout;
use move_vm_types::values::values_impl::Value;
use move_core_types::language_storage::TypeTag;
use move_vm::types::Gas;
use move_vm::types::ScriptTx;
use sp_std::prelude::*;
use move_vm::Vm;
use move_core_types::account_address::AccountAddress;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod mvm;
pub mod storage;
// pub mod error;
// use error::*;

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
         pub VMStorage get(fn vmstorage): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
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
        let mut result = [0_u8; LENGTH];
        let bytes = acc.encode();

        let skip = if bytes.len() < LENGTH {
            LENGTH - bytes.len()
        } else {
            0
        };

        result
            .iter_mut()
            .skip(skip)
            .zip(bytes)
            .for_each(|(dst, src)| {
                *dst = src;
            });
        result
    }
}

impl<T: Trait> Module<T> {
    pub fn get_vm_storage() -> storage::VmStorageAdapter<VMStorage> {
        Default::default()
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
        //   NoneValue,
          /// Errors should have helpful documentation associated with them.
          StorageOverflow,

          MoveScriptTxValidationError,
          MoveScriptTxExecutionError,
          MoveModuleDeployError,
     }
}

// impl<T: Trait> From<VMError> for Error<T> {
//     fn from(_: VMError) -> Self {
//         todo!()
//     }
// }

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
        #[weight = 10_000]
        // TODO: return DispatchResultWithPostInfo with spend gas by vm
        pub fn execute(origin, script_bc: Vec<u8>, args: Vec<u8>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            debug!("executing `execute` with signed {:?}", who);
            // TODO: enable logger for tests
            #[cfg(test)] eprintln!("executing `execute` with signed {:?}", who);

            let vm = mvm::default_vm::<VMStorage>();
            // TODO: gas-table & min-max values shoud be in genesis/config
            let max_gas_amount = (u64::MAX / 1000) - 42;
            // TODO: get native value
            let gas_unit_price = 1;
            let gas = Gas::new(max_gas_amount, gas_unit_price).unwrap();

            let tx = {
                let code: Vec<u8> = script_bc;
                let args = Value::simple_deserialize(
                    &args,
                    &MoveKindInfo::Base(MoveKind::Copyable),
                    &MoveTypeLayout::U64).map(|v|vec![v]).unwrap_or(vec!()
                );
                let type_args: Vec<TypeTag> = Default::default();

                let sender = T::account_as_bytes(&who);
                debug!("converted sender: {:?}", sender);
                #[cfg(test)] eprintln!("converted sender: {:?}", sender);

                let senders: Vec<AccountAddress> = vec![
                    AccountAddress::new(sender),
                ];

                ScriptTx::new(code, args, type_args, senders).map_err(|err|{
                    Error::<T>::MoveScriptTxValidationError
                })?
            };

            let res = vm.execute_script(gas, tx);
            debug!("execution result: {:?}", res);
            #[cfg(test)] eprintln!("execution result: {:?}", res);

            res.map_err(|err|{
            // TODO: unwrap error
            Error::<T>::MoveScriptTxExecutionError
            })?;

            // Emit an event:
            // Self::deposit_event(RawEvent::ResourceMoved(unwrapped_res, who));

            // Return a successful DispatchResult
            Ok(())
        }

        #[weight = 10_000]
        pub fn publish_module(origin, module_bc: Vec<u8>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            debug!("executing `publish` with signed {:?}", who);
            #[cfg(test)] eprintln!("executing `publish` with signed {:?}", who);

            let vm = mvm::default_vm::<VMStorage>();
            // TODO: gas-table & min-max values shoud be in genesis/config
            let max_gas_amount = (u64::MAX / 1000) - 42;
            // TODO: get native value
            let gas_unit_price = 1;
            let gas = Gas::new(max_gas_amount, gas_unit_price).unwrap();

            let tx = {
                use move_vm::types::ModuleTx;

                let code: Vec<u8> = module_bc;
                let sender = T::account_as_bytes(&who);
                debug!("converted sender: {:?}", sender);
                #[cfg(test)] eprintln!("converted sender: {:?}", sender);

                ModuleTx::new(code, AccountAddress::new(sender))
            };

            let res = vm.publish_module(gas, tx);
            debug!("publish result: {:?}", res);
            #[cfg(test)] eprintln!("publish result: {:?}", res);

            res.map_err(|err| Error::<T>::MoveModuleDeployError)?;

            Ok(())
        }

        // fn on_initialize(n: T::BlockNumber,) -> Weight { if n.into() == 42 { panic!("on_initialize") } 7 }
        // fn on_finalize(n: T::BlockNumber,) { if n.into() == 42 { panic!("on_finalize") } }
        // fn on_runtime_upgrade() -> Weight { 10 }
        // fn offchain_worker() {}
     }
}
