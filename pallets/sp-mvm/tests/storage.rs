use sp_core::U256;
use frame_system as system;
use frame_support::assert_ok;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_vm::data::*;
use move_vm::types::ScriptArg;
use move_vm_runtime::data_cache::RemoteCache;
use serde::Deserialize;

pub mod mock;
use mock::*;

fn store_module_bc() -> Vec<u8> {
    include_bytes!("assets/0x1/target/modules/1_Store.mv").to_vec()
}

fn script_bc() -> Vec<u8> {
    include_bytes!("assets/0x1/target/scripts/1_store_u64.mv").to_vec()
}

#[derive(Deserialize)]
struct StoreU64 {
    pub val: u64,
}

fn call_publish_module(origin: Origin, bc: Vec<u8>, mod_name: &str) {
    // execute VM for publish module:
    let result = Mvm::publish_module(origin, bc.clone());
    eprintln!("publish_module result: {:?}", result);
    assert_ok!(result);

    // check storage:
    let store_module_id = ModuleId::new(CORE_CODE_ADDRESS, Identifier::new(mod_name).unwrap());
    let store = Mvm::get_vm_storage();
    let state = State::new(store);
    assert_eq!(bc, state.get_module(&store_module_id).unwrap().unwrap(),);
}

fn call_execute_script(origin: Origin) {
    const TEST_VALUE: u64 = 13;

    // prepare arguments:
    // let args = vec![ScriptArg::U64(TEST_VALUE)];
    let args = vec![TEST_VALUE];

    let result = Mvm::execute(origin, script_bc(), Some(args));
    eprintln!("execute_script result: {:?}", result);
    assert_ok!(result);

    let store = Mvm::get_vm_storage();
    let state = State::new(store);
    let tag = StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new("Store").unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };
    let blob = state
        .get_resource(&CORE_CODE_ADDRESS, &tag)
        .unwrap()
        .unwrap();
    let store: StoreU64 = lcs::from_bytes(&blob).unwrap();
    assert_eq!(TEST_VALUE, store.val);
}

fn address_to_account_id(address: &[u8]) -> <Test as system::Trait>::AccountId {
    let skip = address.len() - core::mem::size_of::<U256>();
    let result = U256::from_big_endian(&address[skip..]);
    println!("ps address: {:?}", result);
    result
}

#[test]
fn publish_module() {
    new_test_ext().execute_with(|| {
        const ADDRESS: AccountAddress = CORE_CODE_ADDRESS;

        let origin = Origin::signed(address_to_account_id(&CORE_CODE_ADDRESS.to_u8()));
        println!("libra: ({}) {:?}", ADDRESS.to_u8().len(), ADDRESS.to_u8());
        println!("ps origin: {:?}", origin);

        call_publish_module(origin, store_module_bc(), "Store")
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        const ADDRESS: AccountAddress = CORE_CODE_ADDRESS;

        let origin = Origin::signed(address_to_account_id(&ADDRESS.to_u8()));
        println!("libra: ({}) {:?}", ADDRESS.to_u8().len(), ADDRESS.to_u8());
        println!("ps origin: {:?}", origin);

        call_publish_module(origin.clone(), store_module_bc(), "Store");
        call_execute_script(origin);
    });
}
