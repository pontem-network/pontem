use move_vm::types::Gas;
use move_vm::types::ScriptTx;
use sp_std::prelude::*;
use frame_support::storage::StorageMap;
use move_vm::mvm::Mvm;
use move_vm::data::EventHandler;

use crate::event::DefaultEventHandler;
use crate::storage::*;

/// Default type of Move VM implementation
pub type DefaultVm<S, E> = Mvm<VmStorageAdapter<S>, E>;

pub type VmWrapperTy<Storage> = VmWrapper<DefaultVm<Storage, DefaultEventHandler>>;

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

impl<Storage> move_vm::Vm for VmWrapperTy<Storage>
where
    Storage: StorageMap<Vec<u8>, Vec<u8>, Query = Option<Vec<u8>>>,
{
    #[inline]
    fn publish_module(
        &self,
        gas: Gas,
        module: move_vm::types::ModuleTx,
    ) -> move_vm::types::VmResult {
        self.0.publish_module(gas, module)
    }

    #[inline]
    fn execute_script(&self, gas: Gas, tx: ScriptTx) -> move_vm::types::VmResult {
        self.0.execute_script(gas, tx)
    }

    #[inline]
    fn clear(&self) {
        self.0.clear()
    }
}

pub trait GetStaticMoveVm<E: EventHandler> {
    type Vm: move_vm::Vm;

    /// Get or create and get the VM
    fn get_move_vm() -> &'static Self::Vm;
}

pub trait CreateMoveVm<T> {
    type Vm: move_vm::Vm;

    fn create_move_vm() -> Self::Vm;

    fn create_move_vm_wrapped() -> VmWrapper<Self::Vm> {
        VmWrapper::new(Self::create_move_vm())
    }
}
