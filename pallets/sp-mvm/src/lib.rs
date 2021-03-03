#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[macro_use]
extern crate log;

use core::convert::TryInto;
use core::convert::TryFrom;
use move_vm::data::ExecutionContext;
use sp_std::prelude::*;
use codec::{FullCodec, FullEncode};
use frame_support::{decl_module, decl_storage, dispatch};
use frame_system::{ensure_signed, ensure_root};
use move_vm::mvm::Mvm;
use move_vm::Vm;
use move_vm::types::Gas;
use move_vm::types::ModuleTx;
use move_vm::types::Transaction;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::CORE_CODE_ADDRESS;

pub mod addr;
pub mod event;
pub mod mvm;
pub mod oracle;
pub mod result;
pub mod storage;

use result::Error;
pub use event::Event;
use event::*;

use storage::MoveVmStorage;
use storage::VmStorageAdapter;

use mvm::TryCreateMoveVm;
use mvm::TryGetStaticMoveVm;
use mvm::TryCreateMoveVmWrapped;
use mvm::VmWrapperTy;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    // type BlockNumber: Into<u64>;
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
     pub struct Module<T: Trait> for enum Call
        where origin: T::Origin,
              T::BlockNumber: TryInto<u64>,
              {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn execute(origin, tx_bc: Vec<u8>) -> dispatch::DispatchResultWithPostInfo {
            let transaction = Transaction::try_from(&tx_bc[..]).map_err(|_| Error::<T>::TransactionValidationError)?;
            let vm = Self::try_get_or_create_move_vm()?;
            let gas = Self::get_move_gas_limit()?;

            let tx = {
                let signers = if transaction.signers_count() == 0 {
                    Vec::with_capacity(0)
                } else if let Ok(account) = ensure_signed(origin) {
                    debug!("executing `execute` with signed {:?}", account);
                    let sender = addr::account_to_bytes(&account);
                    debug!("converted sender: {:?}", sender);
                    vec![AccountAddress::new(sender)]
                } else {
                    // TODO: support multiple signers
                    Vec::with_capacity(0)
                };

                if transaction.signers_count() as usize != signers.len() {
                    error!("Transaction signers num isn't eq signers: {} != {}", transaction.signers_count(), signers.len());
                    return Err(Error::<T>::TransactionSignersNumError.into());
                }

                transaction.into_script(signers).map_err(|_| Error::<T>::TransactionValidationError)?
            };

            let ctx = {
                // TODO: get block, then convert to u64 needed for VM
                // let height = frame_system::Module::<T>::block_number().try_into()?;
                // ExecutionContext::new(height, height)
                ExecutionContext::new(42, 42)
            };
            let res = vm.execute_script(gas, ctx, tx);
            debug!("execution result: {:?}", res);

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(res)?;
            Ok(result)
        }

        #[weight = 10_000]
        pub fn publish(origin, module_bc: Vec<u8>) -> dispatch::DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            debug!("executing `publish` with signed {:?}", who);

            let vm = Self::try_get_or_create_move_vm()?;
            let gas = Self::get_move_gas_limit()?;

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

        #[weight = 10_000]
        pub fn publish_std(origin, module_bc: Vec<u8>) -> dispatch::DispatchResultWithPostInfo {
            ensure_root(origin)?;
            debug!("executing `publish STD` with root");

            let vm = Self::try_get_or_create_move_vm()?;
            let gas = Self::get_move_gas_limit()?;
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
    fn get_move_gas_limit() -> Result<Gas, Error<T>> {
        // TODO: gas-table & min-max values shoud be in genesis/config
        let max_gas_amount = (u64::MAX / 1000) - 42;
        // TODO: get native value
        let gas_unit_price = 1;
        Gas::new(max_gas_amount, gas_unit_price).map_err(|_| Error::InvalidGasAmountMaxValue)
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
    type Vm = Mvm<VmStorageAdapter<VMStorage>, DefaultEventHandler, oracle::DummyOracle>;
    type Error = Error<T>;

    fn try_create_move_vm() -> Result<Self::Vm, Self::Error> {
        trace!("MoveVM created");
        let oracle = Default::default();
        Mvm::new(
            Self::move_vm_storage(),
            Self::create_move_event_handler(),
            oracle,
        )
        .map_err(|err| {
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
            e.addr, e.caller, e.ty_tag, e.message
        );

        // Emit an event:
        Self::deposit_event(RawEvent::Event(e.addr, e.ty_tag, e.message, e.caller));
    }
}
