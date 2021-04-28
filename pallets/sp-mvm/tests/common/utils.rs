#![allow(dead_code)]

use frame_support::dispatch::DispatchResultWithPostInfo as PsResult;
use frame_system as system;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_vm_runtime::data_cache::RemoteCache;
use move_vm::data::*;

use sp_mvm::storage::MoveVmStorage;

use super::assets::*;
use super::mock::*;
use super::addr::*;

pub type AccountId = <Test as system::Config>::AccountId;

/// Publish module __with__ storage check
pub fn publish_module<Asset: BinAsset>(signer: AccountId, module: Asset) {
    publish_module_raw(signer, module.bc().to_vec(), module.name());
}

/// Publish module __without__ storage check
pub fn publish_module_unchecked<Asset: BinAsset>(signer: AccountId, module: Asset) {
    publish_module_raw_unchecked(signer, module.bc().to_vec())
}

pub fn publish_module_raw(signer: AccountId, bc: Vec<u8>, name: &str) {
    publish_module_raw_with_origin_unchecked(Origin::signed(signer), bc.clone());

    // check storage:
    check_storage_mod_raw(signer, bc, name);
}

/// Publish module __without__ storage check
pub fn publish_module_raw_unchecked(signer: AccountId, bc: Vec<u8>) {
    publish_module_raw_with_origin_unchecked(Origin::signed(signer), bc.clone())
}

/// Publish module __without__ storage check
pub fn publish_module_raw_with_origin_unchecked(origin: Origin, bc: Vec<u8>) {
    const GAS_LIMIT: u64 = 1_000_000;

    // execute VM for publish module:
    Mvm::publish_module(origin, bc, GAS_LIMIT).expect("Publish module");
}

/// Publish entire stdlib with sudo/root key
pub fn publish_std() {
    let root = root_ps_acc();
    for module in StdMod::all().into_iter() {
        publish_module_raw(root, module.bc().to_vec(), module.name());
    }
}

pub fn execute_tx_unchecked(origin: Origin, tx: UserTx, gas_limit: u64) -> PsResult {
    // get bytecode:
    let bc = tx.bc().to_vec();
    // execute VM tx:
    let result = Mvm::execute(origin, bc, gas_limit);
    eprintln!("execute tx result: {:?}", result);
    result
}

pub fn check_storage_mod_raw(signer: AccountId, bc: Vec<u8>, name: &str) {
    check_storage_mod_raw_with_addr(to_move_addr(signer), bc, name)
}

pub fn check_storage_mod_raw_with_addr(signer: AccountAddress, bc: Vec<u8>, name: &str) {
    let module_id = ModuleId::new(signer, Identifier::new(name).unwrap());
    let storage = Mvm::move_vm_storage();
    let oracle = MockOracle(None);
    let state = State::new(storage, oracle);
    let stored = state
        .get_module(&module_id)
        .expect("VM state read storage")
        .expect(&format!("Module '{}' should exist", module_id));
    assert_eq!(bc, stored);
}

pub fn check_storage_res<T>(owner: AccountAddress, ty: StructTag, expected: T)
where
    T: for<'de> serde::Deserialize<'de>,
    T: std::cmp::PartialEq + std::fmt::Debug,
{
    let storage = Mvm::move_vm_storage();
    let oracle = MockOracle(None);
    let state = State::new(storage, oracle);
    let blob = state
        .get_resource(&owner, &ty)
        .expect("VM state read storage (resource)")
        .expect(&format!("Resource '{}' should exist", ty));
    let stored: T = bcs::from_bytes(&blob).unwrap();
    assert_eq!(expected, stored);
}
