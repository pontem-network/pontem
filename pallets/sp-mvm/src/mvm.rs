// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! The current file contains code to create Move VM instance.
use move_vm::mvm::Mvm;

use crate::balance::BalancesAdapter;
use crate::storage::*;

/// Default type of Move VM implementation.
pub type DefaultVm<S, E, AccountId, Currencies, CurrencyId> =
    Mvm<StorageAdapter<S>, E, BalancesAdapter<Currencies, AccountId, CurrencyId>>;

/// The trait to create Move VM (returns Result, possible with errors).
pub trait TryCreateMoveVm<T> {
    type Vm: move_vm::Vm;
    type Error;

    /// Get or create and get the VM
    fn try_create_move_vm() -> Result<Self::Vm, Self::Error>;
}

pub use boxed::*;
mod boxed {
    use anyhow::Error;
    use move_vm::StateAccess;
    use move_vm::io::context::ExecutionContext;
    use move_vm::types::Gas;
    use move_vm::types::ScriptTx;

    use crate::balance::boxed::BalancesAdapter;
    use crate::storage::boxed::*;
    use crate::event::DefaultEventHandler;
    use super::{Mvm, TryCreateMoveVm};

    #[cfg(not(feature = "std"))]
    pub use once_cell::race::OnceBox as OnceCell;
    #[cfg(feature = "std")]
    pub use once_cell::sync::OnceCell;

    /// Default type of Move VM implementation
    pub type DefaultVm<E> = Mvm<VmStorageAdapter, E, BalancesAdapter>;
    pub type VmWrapperTy = VmWrapper<DefaultVm<DefaultEventHandler>>;

    /// New-type with unsafe impl Send + Sync.
    /// This is just wrapper around VM without Pin or ref-counting,
    /// so it should only be used between threads.
    /// For thread-local usage or inside the `OnceCell`.
    pub struct VmWrapper<T: move_vm::Vm>(T);
    #[allow(clippy::non_send_fields_in_send_ty)]
    unsafe impl<T: move_vm::Vm> Send for VmWrapper<T> {}
    unsafe impl<T: move_vm::Vm> Sync for VmWrapper<T> {}

    impl StateAccess for VmWrapperTy {
        fn get_module(&self, module_id: &[u8]) -> Result<Option<sp_std::vec::Vec<u8>>, Error> {
            self.0.get_module(module_id)
        }

        fn get_module_abi(
            &self,
            module_id: &[u8],
        ) -> Result<Option<sp_std::vec::Vec<u8>>, Error> {
            self.0.get_module_abi(module_id)
        }

        fn get_resource(
            &self,
            address: &move_core_types::account_address::AccountAddress,
            tag: &[u8],
        ) -> Result<Option<sp_std::vec::Vec<u8>>, Error> {
            self.0.get_resource(address, tag)
        }
    }

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
        fn publish_module(
            &self,
            gas: Gas,
            module: move_vm::types::ModuleTx,
            dry_run: bool,
        ) -> move_vm::types::VmResult {
            self.0.publish_module(gas, module, dry_run)
        }

        #[inline]
        fn publish_module_package(
            &self,
            gas: Gas,
            package: move_vm::types::PublishPackageTx,
            dry_run: bool,
        ) -> move_vm::types::VmResult {
            self.0.publish_module_package(gas, package, dry_run)
        }

        #[inline]
        fn clear(&self) {
            self.0.clear()
        }
    }

    pub trait GetStaticMoveVmCell {
        type Vm: move_vm::Vm;

        fn get_move_vm_cell() -> &'static OnceCell<Self::Vm>;

        fn move_vm_cell_is_inited() -> bool
        where
            <Self as GetStaticMoveVmCell>::Vm: 'static,
        {
            Self::get_move_vm_cell().get().is_some()
        }
    }

    // TODO: auto-impl TryGetStaticMoveVm

    pub trait TryGetStaticMoveVm {
        type Vm: move_vm::Vm;
        type Error;

        /// Get or create and get the VM.
        /// Returns static ref to the VM.
        fn try_get_or_create_move_vm() -> Result<&'static Self::Vm, Self::Error>;
    }

    /// Get or create and get the VM
    pub trait TryCreateMoveVmWrapped<T>: TryCreateMoveVm<T> {
        fn try_create_move_vm_static() -> Result<VmWrapper<Self::Vm>, Self::Error> {
            Self::try_create_move_vm().map(VmWrapper::new)
        }
    }

    impl<T, C: TryCreateMoveVm<T>> TryCreateMoveVmWrapped<T> for C {}

    use core::sync::atomic::{AtomicBool, Ordering};

    /// Usage marker for the VM.
    pub trait MoveVmUsed {
        #[inline(never)]
        fn get_move_vm_used() -> &'static AtomicBool {
            static USED: AtomicBool = AtomicBool::new(false);
            &USED
        }

        fn is_move_vm_used() -> bool {
            Self::get_move_vm_used().load(Ordering::Relaxed)
        }

        fn set_move_vm_used() {
            Self::get_move_vm_used().store(true, Ordering::Relaxed);
        }
        fn set_move_vm_clean() {
            Self::get_move_vm_used().store(false, Ordering::Relaxed);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::MoveVmUsed;

        struct Vm;

        impl MoveVmUsed for Vm {}

        #[test]
        fn usage_marker() {
            assert_eq!(false, Vm::is_move_vm_used());

            Vm::set_move_vm_used();
            assert_eq!(true, Vm::is_move_vm_used());

            Vm::set_move_vm_clean();
            assert_eq!(false, Vm::is_move_vm_used());
        }
    }
}
