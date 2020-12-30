use frame_system as system;
use frame_support::assert_ok;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_vm::data::*;
use move_vm_runtime::data_cache::RemoteCache;

mod mock;
use mock::*;

mod utils;
use utils::*;

fn event_module_bc() -> Vec<u8> {
    include_bytes!("assets/target/modules/0_Event.mv").to_vec()
}
fn vec_module_bc() -> Vec<u8> {
    include_bytes!("assets/target/modules/1_Vector.mv").to_vec()
}

fn script_bc() -> Vec<u8> {
    include_bytes!("assets/target/scripts/0_emit_event.mv").to_vec()
}

fn call_publish_module(signer: <Test as system::Trait>::AccountId, bc: Vec<u8>, mod_name: &str) {
    let origin = Origin::signed(signer);
    // execute VM for publish module:
    let result = Mvm::publish_module(origin, bc.clone());
    eprintln!("publish_module result: {:?}", result);
    assert_ok!(result);

    // check storage:
    let module_id = ModuleId::new(to_move_addr(signer), Identifier::new(mod_name).unwrap());
    let storage = Mvm::get_vm_storage();
    let state = State::new(storage);
    assert_eq!(bc, state.get_module(&module_id).unwrap().unwrap());
}

fn call_execute_script(origin: Origin) {
    const TEST_VALUE: u64 = 42;

    // prepare arguments:
    // let args = vec![ScriptArg::U64(TEST_VALUE)];
    let args = vec![TEST_VALUE];

    // execute VM tx:
    let result = Mvm::execute(origin, script_bc(), Some(args));
    eprintln!("result: {:?}", result);
    assert_ok!(result);

    // TODO: check storage for event...
}

#[test]
fn publish_module() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();

        call_publish_module(root, vec_module_bc(), "Vector");
        call_publish_module(root, event_module_bc(), "Event");
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = Origin::signed(origin_ps_acc());

        call_publish_module(root, vec_module_bc(), "Vector");
        call_publish_module(root, event_module_bc(), "Event");
        call_execute_script(origin);
    });
}
