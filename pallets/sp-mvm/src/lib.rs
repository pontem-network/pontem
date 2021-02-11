#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[macro_use]
extern crate log;

use sp_std::prelude::*;
use codec::{FullCodec, FullEncode};
use frame_support::{decl_module, decl_storage, dispatch};
use frame_system::{ensure_signed, ensure_root};
use move_vm::mvm::Mvm;
use move_vm::Vm;
use move_vm::types::Gas;
use move_vm::types::ModuleTx;
use move_vm::types::ScriptTx;
use move_vm::types::ScriptArg;
use move_core_types::language_storage::TypeTag;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::CORE_CODE_ADDRESS;

pub mod addr;
pub mod event;
pub mod gas;
pub mod mvm;
pub mod result;
pub mod storage;

use result::Error;
use crate::gas::GasWeightMapping;
pub use event::Event;
use event::*;

use storage::MoveVmStorage;
use storage::VmStorageAdapter;

use mvm::TryCreateMoveVm;
use mvm::TryGetStaticMoveVm;
use mvm::TryCreateMoveVmWrapped;
use mvm::VmWrapperTy;

const GAS_UNIT_PRICE: u64 = 1;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    // Gas settings.
    type GasWeightMapping: gas::GasWeightMapping;
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
         pub VMStorage: map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
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

        // Temprorally args changed to just u64 numbers because of troubles with codec & web-client...
        // They should be this: Option<Vec<ScriptArg>> ,ty_args: Vec<TypeTag>
        #[weight = T::GasWeightMapping::gas_to_weight(*gas_limit)]
        pub fn execute(origin, script_bc: Vec<u8>, args: Option<Vec<u64>>, gas_limit: u64) -> dispatch::DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            debug!("executing `execute` with signed {:?}", who);

            let vm = Self::try_get_or_create_move_vm()?;
            let gas = Self::get_move_gas_limit(gas_limit)?;

            let tx = {
                let type_args: Vec<TypeTag> = Default::default();

                let args = args.map(|vec|
                        vec.into_iter().map(ScriptArg::U64).collect()
                ).unwrap_or_default();

                let sender = addr::account_to_bytes(&who);
                debug!("converted sender: {:?}", sender);

                let senders: Vec<AccountAddress> = vec![
                    AccountAddress::new(sender),
                ];

                ScriptTx::new(script_bc, args, type_args, senders).map_err(|_|{
                    Error::<T>::ScriptValidationError
                })?
            };

            let res = vm.execute_script(gas, tx);
            debug!("execution result: {:?}", res);

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(res)?;
            Ok(result)
        }

        #[weight = T::GasWeightMapping::gas_to_weight(*gas_limit)]
        pub fn publish_module(origin, module_bc: Vec<u8>, gas_limit: u64) -> dispatch::DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            debug!("executing `publish` with signed {:?}", who);

            let vm = Self::try_get_or_create_move_vm()?;
            let gas = Self::get_move_gas_limit(gas_limit)?;

            let tx = {
                let sender = addr::account_to_bytes(&who);
                debug!("converted sender: {:?}", sender);

                ModuleTx::new(module_bc, AccountAddress::new(sender))
            };

            let res = vm.publish_module(gas, tx);
            debug!("publish result: {:?}", res);

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(res)?;

            // Emit an event:
            Self::deposit_event(RawEvent::ModulePublished(who));

            Ok(result)
        }

        #[weight = T::GasWeightMapping::gas_to_weight(*gas_limit)]
        pub fn publish_std(origin, module_bc: Vec<u8>, gas_limit: u64) -> dispatch::DispatchResultWithPostInfo {
            ensure_root(origin)?;
            debug!("executing `publish STD` with root");

            let vm = Self::try_get_or_create_move_vm()?;
            let gas = Self::get_move_gas_limit(gas_limit)?;
            let tx = ModuleTx::new(module_bc, CORE_CODE_ADDRESS);
            let res = vm.publish_module(gas, tx);
            debug!("publish result: {:?}", res);

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(res)?;

            // Emit an event:
            Self::deposit_event(RawEvent::StdModulePublished);

            Ok(result)
        }

        fn on_finalize(n: T::BlockNumber) {
            Self::try_get_or_create_move_vm().unwrap().clear();
            trace!("MoveVM cleared on {:?}", n);
        }
     }
}

impl<T: Trait> Module<T> {
    fn get_move_gas_limit(gas_limit: u64) -> Result<Gas, Error<T>> {
        Gas::new(gas_limit, GAS_UNIT_PRICE).map_err(|_| Error::InvalidGasAmountMaxValue)
    }
}

/// Get storage adapter ready for the VM
impl<T: Trait, K, V> MoveVmStorage<T, K, V> for Module<T>
where
    K: FullEncode,
    V: FullCodec,
{
    type VmStorage = VMStorage;
}

impl<T: Trait> TryCreateMoveVm<T> for Module<T> {
    type Vm = Mvm<VmStorageAdapter<VMStorage>, DefaultEventHandler>;
    type Error = Error<T>;

    fn try_create_move_vm() -> Result<Self::Vm, Self::Error> {
        trace!("MoveVM created");
        Mvm::new(Self::move_vm_storage(), Self::create_move_event_handler()).map_err(|err| {
            error!("{}", err);
            Error::InvalidVMConfig
        })
    }
}

impl<T: Trait> TryGetStaticMoveVm<DefaultEventHandler> for Module<T> {
    type Vm = VmWrapperTy<VMStorage>;
    type Error = Error<T>;

    fn try_get_or_create_move_vm() -> Result<&'static Self::Vm, Self::Error> {
        #[cfg(not(feature = "std"))]
        use once_cell::race::OnceBox as OnceCell;
        #[cfg(feature = "std")]
        use once_cell::sync::OnceCell;

        static VM: OnceCell<VmWrapperTy<VMStorage>> = OnceCell::new();
        VM.get_or_try_init(|| {
            #[cfg(feature = "std")]
            {
                Self::try_create_move_vm_wrapped()
            }
            #[cfg(not(feature = "std"))]
            Self::try_create_move_vm_wrapped().map(Box::from)
        })
    }
}

impl<T: Trait> DepositMoveEvent for Module<T> {
    fn deposit_move_event(e: MoveEventArguments) {
        debug!(
            "MoveVM Event: {:?} {:?} {:?} {:?}",
            e.guid, e.seq_num, e.ty_tag, e.message
        );

        // Emit an event:
        Self::deposit_event(RawEvent::Event(e.guid, e.seq_num, e.ty_tag, e.message));
    }
}
