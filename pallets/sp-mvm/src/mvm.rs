use sp_std::prelude::*;
use codec::{FullCodec, FullEncode};
use frame_support::storage::StorageMap;
use move_vm::mvm::Mvm;
use move_vm::data::EventHandler;

use crate::storage::*;

/// Default type of Move VM implementation
pub type DefaultVm<T, E> = Mvm<VmStorageAdapter<T>, E>;

/// Create or get cached default VM
pub fn default_vm<T, E>(event_handler: E) -> impl move_vm::Vm
where
    T: StorageMap<Vec<u8>, Vec<u8>, Query = Option<Vec<u8>>>,
    E: EventHandler,
{
    let store: VmStorageAdapter<T> = Default::default();
    // let event_handler = E::default();
    Mvm::new(store, event_handler)
}

/// Move VM builder/getter trait
trait MoveVm<T: frame_system::Trait, K: FullEncode, V: FullCodec, E: EventHandler> {
    type Vm: move_vm::Vm;
    fn move_vm(event_handler: E) -> Self::Vm;
}

// default impl
impl<T, K, V, E> MoveVm<T, K, V, E> for T
where
    T: frame_system::Trait,
    T: StorageMap<Vec<u8>, Vec<u8>, Query = Option<Vec<u8>>>,
    K: FullEncode,
    V: FullCodec,
    E: EventHandler,
{
    type Vm = Mvm<VmStorageAdapter<T, Vec<u8>, Vec<u8>>, E>;

    fn move_vm(event_handler: E) -> Self::Vm {
        Mvm::new(Default::default(), event_handler)
    }
}
