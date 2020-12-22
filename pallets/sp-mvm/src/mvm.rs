use sp_std::prelude::*;
use codec::FullCodec;
use codec::FullEncode;
use move_vm::dvm::Dvm;
use frame_support::storage::StorageMap;

use crate::storage::*;

/// Default type of Move VM implementation
pub type DefaultVm<T> = Dvm<VmStorageAdapter<T>>;

/// Create or get cached default VM
pub fn default_vm<T>() -> impl move_vm::Vm
where
    T: StorageMap<Vec<u8>, Vec<u8>, Query = Option<Vec<u8>>>,
{
    let store: VmStorageAdapter<T> = Default::default();
    Dvm::new(store)
}

/// Move VM builder/getter trait
trait MoveVm<T: frame_system::Trait, K: FullEncode, V: FullCodec> {
    type Vm: move_vm::Vm;
    fn move_vm() -> Self::Vm;
}

// default impl
impl<T, K, V> MoveVm<T, K, V> for T
where
    T: frame_system::Trait,
    T: StorageMap<Vec<u8>, Vec<u8>, Query = Option<Vec<u8>>>,
    K: FullEncode,
    V: FullCodec,
{
    type Vm = Dvm<VmStorageAdapter<T, Vec<u8>, Vec<u8>>>;

    fn move_vm() -> Self::Vm {
        Dvm::new(Default::default())
    }
}
