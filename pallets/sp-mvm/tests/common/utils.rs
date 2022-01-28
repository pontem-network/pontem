#![allow(dead_code)]

use std::convert::TryFrom;
use frame_support::dispatch::DispatchResultWithPostInfo as PsResult;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_core_types::language_storage::TypeTag;
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
    let bytecode = module.bytes().to_vec();
    let name = module.name();
    let result = publish_module_unchecked(signer, module, gas_limit)?;
    check_storage_module(to_move_addr(signer), bytecode, name);
    Ok(result)
}

/// Publish module __without__ storage check
pub fn publish_module_unchecked(
    signer: AccountId,
    module: &Asset,
    gas_limit: Option<u64>,
) -> PsResult {
    let gas_limit = gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);
    Mvm::publish_module(Origin::signed(signer), module.bytes().to_vec(), gas_limit)
}

/// Publish module as root __without__ storage check
pub fn publish_module_as_root_unchecked(module: &Asset, gas_limit: Option<u64>) -> PsResult {
    let gas_limit = gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);
    Mvm::publish_module(Origin::root(), module.bytes().to_vec(), gas_limit)
}

/// Publish module as root __with__ storage check
pub fn publish_module_as_root(module: &Asset, gas_limit: Option<u64>) -> PsResult {
    let bytecode = module.bytes().to_vec();
    let name = module.name();
    let result = publish_module_as_root_unchecked(module, gas_limit)?;
    check_storage_module(CORE_CODE_ADDRESS, bytecode, name);
    Ok(result)
}

/// Publish package.
///
/// Publish package __with__ storage check
pub fn publish_package(signer: AccountId, package: &Package, gas_limit: Option<u64>) -> PsResult {
    let bytecode = package.bytes().to_vec();
    let names = package.modules();
    let result = publish_package_unchecked(signer, package, gas_limit)?;
    check_storage_package(to_move_addr(signer), bytecode, names);
    Ok(result)
}

/// Publish package as root __with__ storage check.
pub fn publish_package_as_root(package: &Package, gas_limit: Option<u64>) -> PsResult {
    let bytecode = package.bytes().to_vec();
    let names = package.modules();
    let result = publish_package_unchecked_as_root(package, gas_limit)?;
    check_storage_package(CORE_CODE_ADDRESS, bytecode, names);
    Ok(result)
}

/// Publish package as root __without__ storage checks.
pub fn publish_package_unchecked_as_root(package: &Package, gas_limit: Option<u64>) -> PsResult {
    let gas_limit = gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);
    Mvm::publish_package(Origin::root(), package.bytes().to_vec(), gas_limit)
}

/// Publish package __without__ storage check
pub fn publish_package_unchecked(
    signer: AccountId,
    package: &Package,
    gas_limit: Option<u64>,
) -> PsResult {
    let gas_limit = gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);
    Mvm::publish_package(Origin::signed(signer), package.bytes().to_vec(), gas_limit)
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

/// Returns TypeTag 0x1::PONT::T
pub fn get_type_tag_pont_coin() -> StructTag {
    StructTag {
        address: ROOT_ADDR,
        module: Identifier::new("PONT").unwrap(),
        name: Identifier::new("PONT").unwrap(),
        type_params: vec![],
    }
}

/// Returns TypeTag Pontem::T<0x1::PONT::T>
pub fn get_type_tag_pont_res() -> StructTag {
    StructTag {
        address: ROOT_ADDR,
        module: Identifier::new("Diem").unwrap(),
        name: Identifier::new("Diem").unwrap(),
        type_params: vec![TypeTag::Struct(get_type_tag_pont_coin())],
    }
}

/// Returns TypeTag 0x1::Account::Balance<0x1::PONT::T>
pub fn get_type_tag_balance_pont() -> StructTag {
    StructTag {
        address: ROOT_ADDR,
        module: Identifier::new("DiemAccount").unwrap(),
        name: Identifier::new("Balance").unwrap(),
        type_params: vec![TypeTag::Struct(get_type_tag_pont_coin())],
    }
}
