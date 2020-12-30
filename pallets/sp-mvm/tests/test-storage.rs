use frame_support::assert_ok;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_vm::data::*;
use move_vm_runtime::data_cache::RemoteCache;
use serde::Deserialize;

mod mock;
use mock::*;

mod consts;
use consts::*;

fn store_module_bc() -> Vec<u8> {
    include_bytes!("assets/target/modules/2_Store.mv").to_vec()
}

fn script_bc() -> Vec<u8> {
    include_bytes!("assets/target/scripts/1_store_u64.mv").to_vec()
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
    let store_module_id = ModuleId::new(origin_move_addr(), Identifier::new(mod_name).unwrap());
    let store = Mvm::get_vm_storage();
    let state = State::new(store);
    assert_eq!(bc, state.get_module(&store_module_id).unwrap().unwrap());
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
        address: origin_move_addr(),
        module: Identifier::new("Store").unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };
    let blob = state
        .get_resource(&origin_move_addr(), &tag)
        .unwrap()
        .unwrap();
    let store: StoreU64 = lcs::from_bytes(&blob).unwrap();
    assert_eq!(TEST_VALUE, store.val);
}

#[test]
fn publish_module() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(origin_ps_acc());

        call_publish_module(origin, store_module_bc(), "Store")
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(origin_ps_acc());

        call_publish_module(origin.clone(), store_module_bc(), "Store");
        call_execute_script(origin);
    });
}
