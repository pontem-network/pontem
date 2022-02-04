#![allow(dead_code)]

use std::convert::TryFrom;
use frame_support::dispatch::DispatchResultWithPostInfo as PsResult;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_vm::io::state::State;
use move_vm::types::ModulePackage;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_core_types::resolver::ModuleResolver;
use move_core_types::resolver::ResourceResolver;

use sp_mvm::storage::MoveVmStorage;

use super::assets::*;
use super::mock::*;
use super::addr::*;

pub type AccountId = <Test as frame_system::Config>::AccountId;

const DEFAULT_GAS_LIMIT: u64 = 1_000_000;

/// Publish module __with__ storage check
pub fn publish_module(signer: AccountId, module: &Asset, gas_limit: Option<u64>) -> PsResult {
    let result = Mvm::publish_module(
        Origin::signed(signer),
        module.bytes().to_vec(),
        gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
    )?;
    check_storage_module(to_move_addr(signer), module.bytes().to_vec(), module.name());
    Ok(result)
}

/// Publish module as root __with__ storage check
pub fn publish_module_as_root(module: &Asset, gas_limit: Option<u64>) -> PsResult {
    let result = Mvm::publish_module(
        Origin::root(),
        module.bytes().to_vec(),
        gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
    )?;
    check_storage_module(CORE_CODE_ADDRESS, module.bytes().to_vec(), module.name());
    Ok(result)
}

/// Publish package.
///
/// Publish package __with__ storage check
pub fn publish_package(signer: AccountId, package: &Package, gas_limit: Option<u64>) -> PsResult {
    let result = Mvm::publish_package(
        Origin::signed(signer),
        package.bytes().to_vec(),
        gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
    )?;
    check_storage_package(
        to_move_addr(signer),
        package.bytes().to_vec(),
        package.modules(),
    );
    Ok(result)
}

/// Publish package as root __with__ storage check.
pub fn publish_package_as_root(package: &Package, gas_limit: Option<u64>) -> PsResult {
    let result = Mvm::publish_package(
        Origin::root(),
        package.bytes().to_vec(),
        gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
    )?;
    check_storage_package(
        CORE_CODE_ADDRESS,
        package.bytes().to_vec(),
        package.modules(),
    );
    Ok(result)
}

/// Execute transaction script.
pub fn execute_tx(origin: AccountId, tx: &Asset, gas_limit: Option<u64>) -> PsResult {
    let gas_limit = gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);
    // get bytecode:
    let bc = tx.bytes().to_vec();
    // execute VM tx:
    let result = Mvm::execute(Origin::signed(origin), bc, gas_limit);
    eprintln!("execute tx result: {:?}", result);
    result
}

pub fn check_storage_module<Bc: AsRef<[u8]>>(
    account_address: AccountAddress,
    bc: Bc,
    name: &str,
) {
    let module_id = ModuleId::new(account_address, Identifier::new(name).unwrap());
    let storage = Mvm::move_vm_storage();
    let state = State::new(storage);
    let stored = state
        .get_module(&module_id)
        .expect("VM state read storage")
        .expect(&format!("Module '{}' should exist", module_id));
    assert_eq!(bc.as_ref(), &stored);
}

pub fn check_storage_package<Bc: AsRef<[u8]>>(
    account_address: AccountAddress,
    bytecode: Bc,
    names: &[&str],
) {
    let (modules, _) = ModulePackage::try_from(bytecode.as_ref())
        .unwrap()
        .into_tx(ROOT_ADDR)
        .into_inner();

    for (i, bytecode) in modules.iter().enumerate() {
        check_storage_module(account_address, bytecode, names[i]);
    }
}

pub fn check_storage_res<T>(owner: AccountAddress, ty: StructTag, expected: T)
where
    T: for<'de> serde::Deserialize<'de>,
    T: std::cmp::PartialEq + std::fmt::Debug,
{
    let storage = Mvm::move_vm_storage();
    let state = State::new(storage);
    let blob = state
        .get_resource(&owner, &ty)
        .expect("VM state read storage (resource)")
        .expect(&format!("Resource '{}' should exist", ty));

    let tt_str = format!("{}", ty);
    println!("checking stored resource '{}'", tt_str);
    let stored: T =
        bcs::from_bytes(&blob).expect(&format!("Resource '{}' should exists", tt_str));
    assert_eq!(expected, stored);
}
