use move_vm::types::Gas;
use move_vm::types::ScriptTx;
use move_vm::data::EventHandler;
use move_vm::mvm::Mvm;

use crate::balance::BalancesAdapter;
use crate::event::DefaultEventHandler;
use crate::storage::*;

/// Default type of Move VM implementation
pub type DefaultVm<S, E, O, R> = Mvm<VmStorageAdapter<S>, E, O, BalancesAdapter<R>>;

pub trait CreateMoveVm<T> {
    type Vm: move_vm::Vm;

    /// Create VM instance
    fn create_move_vm() -> Self::Vm;
}

pub trait TryCreateMoveVm<T> {
    type Vm: move_vm::Vm;
    type Error;

    /// Get or create and get the VM
    fn try_create_move_vm() -> Result<Self::Vm, Self::Error>;
}

#[cfg(not(feature = "no-vm-static"))]
pub use vm_static::*;
#[cfg(not(feature = "no-vm-static"))]
mod vm_static {
    use move_vm::data::ExecutionContext;

    use crate::oracle::DummyOracle;
    use crate::storage::boxed::*;
    use crate::balance::boxed::BalancesAdapter;
    use super::{
        EventHandler, Gas, Mvm, CreateMoveVm, ScriptTx, TryCreateMoveVm, DefaultEventHandler,
    };

    /// Default type of Move VM implementation
    pub type DefaultVm<E, O> = Mvm<VmStorageAdapter, E, O, BalancesAdapter>;
    pub type VmWrapperTy = VmWrapper<DefaultVm<DefaultEventHandler, DummyOracle>>;

    /// New-type with unsafe impl Send + Sync.
    /// This is just wrapper around VM without Pin or ref-counting,
    /// so it should only be used between threads.
    /// For thread-local usage or inside the `OnceCell`.
    pub struct VmWrapper<T: move_vm::Vm>(T);
    unsafe impl<T: move_vm::Vm> Send for VmWrapper<T> {}
    unsafe impl<T: move_vm::Vm> Sync for VmWrapper<T> {}

    impl<T: move_vm::Vm> VmWrapper<T> {
        pub fn new(vm: T) -> Self {
            Self(vm)
        }
    }

    impl<T: move_vm::Vm> AsRef<T> for VmWrapper<T> {
        fn as_ref(&self) -> &T {
            &self.0
        }
    }

    impl<T: move_vm::Vm> AsMut<T> for VmWrapper<T> {
        fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl move_vm::Vm for VmWrapperTy {
        #[inline]
        fn publish_module(
            &self,
            gas: Gas,
            module: move_vm::types::ModuleTx,
            dry_run: bool,
        ) -> move_vm::types::VmResult {
            self.0.publish_module(gas, module, dry_run)
        }

        #[inline]
        fn execute_script(
            &self,
            gas: Gas,
            ctx: ExecutionContext,
            tx: ScriptTx,
            dry_run: bool,
        ) -> move_vm::types::VmResult {
            self.0.execute_script(gas, ctx, tx, dry_run)
        }

        #[inline]
        fn clear(&self) {
            self.0.clear()
        }
    }

    /**
    # Impl example

    ```no_run,ignore
    decl_storage! {
        trait Store for Module<T: Trait> as VM {
            /// Storage for move- write-sets contains code & resources
            pub Storage: map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
        }
    }

    impl<T: Trait> GetStaticMoveVm<MyEventHandler> for Module<T> {
        type Vm = VmWrapperTy<Storage>;

        fn get_or_create_move_vm() -> &'static Self::Vm {
            #[cfg(not(feature = "std"))]
            use once_cell::race::OnceBox as OnceCell;
            #[cfg(feature = "std")]
            use once_cell::sync::OnceCell;

            static VM: OnceCell<VmWrapperTy<Storage>> = OnceCell::new();

            // there .into() needed one-cell's OnceBox to
            // into Box implicitly convertion for no-std
            // into itself (noop) for std/test
            #[allow(clippy::useless_conversion)]
            VM.get_or_init(|| Self::create_move_vm_wrapped().into())
        }
    }
    ```
    */
    pub trait GetStaticMoveVm<E: EventHandler> {
        type Vm: move_vm::Vm;

        /// Get or create and get the VM
        fn get_or_create_move_vm() -> &'static Self::Vm;
    }

    pub trait CreateMoveVmWrapped<T>: CreateMoveVm<T> {
        fn create_move_vm_wrapped() -> VmWrapper<Self::Vm> {
            VmWrapper::new(Self::create_move_vm())
        }
    }

    impl<T, C: CreateMoveVm<T>> CreateMoveVmWrapped<T> for C {}

    pub trait TryGetStaticMoveVm<E: EventHandler> {
        type Vm: move_vm::Vm;
        type Error;

        /// Get or create and get the VM.
        /// Returns static ref to the VM.
        fn try_get_or_create_move_vm() -> Result<&'static Self::Vm, Self::Error>;
    }

    /// Get or create and get the VM
    pub trait TryCreateMoveVmWrapped<T>: TryCreateMoveVm<T> {
        fn try_create_move_vm_wrapped() -> Result<VmWrapper<Self::Vm>, Self::Error> {
            Self::try_create_move_vm().map(VmWrapper::new)
        }
    }

    impl<T, C: TryCreateMoveVm<T>> TryCreateMoveVmWrapped<T> for C {}
}
