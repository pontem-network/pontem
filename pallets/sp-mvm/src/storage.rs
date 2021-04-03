use sp_std::prelude::*;
use codec::FullCodec;
use codec::FullEncode;
use move_vm::data::Storage;
use frame_support::storage::StorageMap;

// TODO: mb. use StoredMap

pub trait MoveVmStorage<T, K: FullEncode, V: FullCodec> {
    type VmStorage;

    fn move_vm_storage() -> VmStorageAdapter<Self::VmStorage, K, V>
    where
        // TODO: simplify to
        // Self::VmStorage: StorageMap<K, V>,
        Self::VmStorage: StorageMap<K, V, Query = Option<V>>,
    {
        Default::default()
    }
}

/// Vm storage adapter for native storage
pub struct VmStorageAdapter<T, K = Vec<u8>, V = Vec<u8>>(core::marker::PhantomData<(T, K, V)>);

impl<T, K, V> Default for VmStorageAdapter<T, K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Default VM storage implementation
impl<T: StorageMap<Vec<u8>, Vec<u8>, Query = Option<Vec<u8>>>> Storage
    for VmStorageAdapter<T, Vec<u8>, Vec<u8>>
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        #[cfg(feature = "std")]
        trace!("storage::get {:?}", key);
        T::get(key)
    }

    fn insert(&self, key: &[u8], value: &[u8]) {
        trace!("storage::set {:?} <= {} bytes", key, value.len());
        T::insert(key, value)
    }

    fn remove(&self, key: &[u8]) {
        trace!("storage::rem {:?}", key);
        T::remove(key)
    }
}
