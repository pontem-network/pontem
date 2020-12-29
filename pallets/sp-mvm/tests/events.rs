use frame_system as system;
use frame_support::assert_ok;
use sp_core::U256;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_core_types::language_storage::ModuleId;
use move_vm::data::*;
use move_vm::types::ScriptArg;
use move_vm_runtime::data_cache::RemoteCache;

mod mock;
use mock::*;

fn store_module_bc() -> Vec<u8> {
    include_bytes!("../tests/assets/target/modules/0_Event.mv").to_vec()
}

fn script_bc() -> Vec<u8> {
    include_bytes!("../tests/assets/target/scripts/0_emit_event.mv").to_vec()
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
    const TEST_VALUE: u64 = 42;

    // prepare arguments:
    let args = vec![ScriptArg::U64(TEST_VALUE)];

    // execute VM tx:
    let result = Mvm::execute(origin, script_bc(), args);
    eprintln!("result: {:?}", result);
    assert_ok!(result);

    // TODO: check storage for event...
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

        call_publish_module(origin, store_module_bc(), "Event")
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        const ADDRESS: AccountAddress = CORE_CODE_ADDRESS;

        let origin = Origin::signed(address_to_account_id(&ADDRESS.to_u8()));
        println!("libra: ({}) {:?}", ADDRESS.to_u8().len(), ADDRESS.to_u8());
        println!("ps origin: {:?}", origin);

        call_publish_module(origin.clone(), store_module_bc(), "Event");
        call_execute_script(origin);
    });
}
