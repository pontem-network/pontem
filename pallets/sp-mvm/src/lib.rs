#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[macro_use]
extern crate log;

use sp_std::prelude::*;
use frame_support::{decl_module, decl_storage, dispatch};
use frame_system::ensure_signed;
use move_vm::Vm;
use move_vm::types::Gas;
use move_vm::types::ScriptTx;
use move_vm::types::ScriptArg;
use move_core_types::language_storage::TypeTag;
use move_core_types::account_address::AccountAddress;

pub mod addr;
pub mod event;
pub mod mvm;
pub mod result;
pub mod storage;

use result::Error;
use addr::AccountIdAsBytes;
pub use event::Event;

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

impl<T: Trait> Module<T> {
    pub fn get_vm_storage() -> storage::VmStorageAdapter<VMStorage> {
        Default::default()
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000]
        // Temprorally args changed to just u64 numbers because of troubles with codec & web-client...
        // They should be this: Option<Vec<ScriptArg>> ,ty_args: Vec<TypeTag>
        pub fn execute(origin, script_bc: Vec<u8>, args: Option<Vec<u64>>) -> dispatch::DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            debug!("executing `execute` with signed {:?}", who);
            // TODO: enable logger for tests
            #[cfg(feature = "std")] eprintln!("executing `execute` with signed {:?}", who);

            let event_handler = event::EventWriter::new(Self::deposit_event);
            let vm = mvm::default_vm::<VMStorage, _>(event_handler);
            // TODO: gas-table & min-max values shoud be in genesis/config
            let max_gas_amount = (u64::MAX / 1000) - 42;
            // TODO: get native value
            let gas_unit_price = 1;
            let gas = Gas::new(max_gas_amount, gas_unit_price).unwrap();

            let tx = {
                let code: Vec<u8> = script_bc;
                let type_args: Vec<TypeTag> = Default::default();

                let args = args.map(|vec|
                        vec.into_iter().map(|v|ScriptArg::U64(v)
                    ).collect()
                ).unwrap_or_else(||vec![]);

                let sender = T::account_to_bytes(&who);
                debug!("converted sender: {:?}", sender);
                #[cfg(feature = "std")] eprintln!("converted sender: {:?}", sender);

                let senders: Vec<AccountAddress> = vec![
                    AccountAddress::new(sender),
                ];

                ScriptTx::new(code, args, type_args, senders).map_err(|_|{
                    Error::<T>::ScriptValidationError
                })?
            };

            let res = vm.execute_script(gas, tx);
            debug!("execution result: {:?}", res);
            #[cfg(feature = "std")] eprintln!("execution result: {:?}", res);

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(res)?;
            Ok(result)
        }

        #[weight = 10_000]
        pub fn publish_module(origin, module_bc: Vec<u8>) -> dispatch::DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            debug!("executing `publish` with signed {:?}", who);
            #[cfg(test)] eprintln!("executing `publish` with signed {:?}", who);

            let event_handler = event::EventWriter::new(Self::deposit_event);
            let vm = mvm::default_vm::<VMStorage, _>(event_handler);
            // TODO: gas-table & min-max values shoud be in genesis/config
            let max_gas_amount = (u64::MAX / 1000) - 42;
            // TODO: get native value
            let gas_unit_price = 1;
            let gas = Gas::new(max_gas_amount, gas_unit_price).unwrap();

            let tx = {
                use move_vm::types::ModuleTx;

                let code: Vec<u8> = module_bc;
                let sender = T::account_to_bytes(&who);
                debug!("converted sender: {:?}", sender);
                #[cfg(test)] eprintln!("converted sender: {:?}", sender);

                ModuleTx::new(code, AccountAddress::new(sender))
            };

            let res = vm.publish_module(gas, tx);
            debug!("publish result: {:?}", res);
            #[cfg(test)] eprintln!("publish result: {:?}", res);

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(res)?;

            // Emit an event:
            Self::deposit_event(event::RawEvent::ModulePublished(who));

            Ok(result)
        }
     }
}
