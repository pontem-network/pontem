use frame_system as system;
use frame_support::assert_ok;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_core_types::language_storage::TypeTag;
use move_vm_runtime::data_cache::RemoteCache;
use move_vm::data::*;

use sp_mvm::storage::MoveVmStorage;
use sp_mvm::event::MoveRawEvent as RawEvent;

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

fn script_tx() -> Vec<u8> {
    include_bytes!("assets/target/transactions/emit_event.mvt").to_vec()
}

fn call_publish_module_with_origin(origin: Origin, bc: Vec<u8>) {
    // execute VM for publish module:
    let result = Mvm::publish(origin, bc);
    eprintln!("publish_module result: {:?}", result);
    assert_ok!(result);
}

fn call_publish_module(signer: <Test as system::Trait>::AccountId, bc: Vec<u8>, mod_name: &str) {
    let origin = Origin::signed(signer);
    call_publish_module_with_origin(origin, bc.clone());

    // check storage:
    check_storage(signer, bc, mod_name);
}

fn check_storage(signer: <Test as system::Trait>::AccountId, bc: Vec<u8>, mod_name: &str) {
    // check storage:
    let module_id = ModuleId::new(to_move_addr(signer), Identifier::new(mod_name).unwrap());
    let storage = Mvm::move_vm_storage();
    let state = State::new(storage);
    assert_eq!(bc, state.get_module(&module_id).unwrap().unwrap());
}

fn check_storage_with_addr(signer: AccountAddress, bc: Vec<u8>, mod_name: &str) {
    // check storage:
    let module_id = ModuleId::new(signer, Identifier::new(mod_name).unwrap());
    let storage = Mvm::move_vm_storage();
    let state = State::new(storage);
    assert_eq!(bc, state.get_module(&module_id).unwrap().unwrap());
}

fn call_execute_script(origin: Origin) {
    // execute VM tx:
    let result = Mvm::execute(origin, script_tx());
    eprintln!("result: {:?}", result);
    assert_ok!(result);

    // TODO: check storage for event...
}

#[test]
/// publish modules personally as root
fn publish_module() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        call_publish_module(root, vec_module_bc(), "Vector");
        call_publish_module(root, event_module_bc(), "Event");
    });
}

#[test]
#[ignore = "FIXME: Origin::root() produces BadOrigin"]
/// publish std modules as root
fn publish_module_as_root() {
    new_test_ext().execute_with(|| {
        call_publish_module_with_origin(Origin::root(), vec_module_bc());
        check_storage_with_addr(CORE_CODE_ADDRESS, vec_module_bc(), "Vector");

        call_publish_module_with_origin(Origin::root(), event_module_bc());
        check_storage_with_addr(CORE_CODE_ADDRESS, vec_module_bc(), "Event");
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = Origin::signed(origin_ps_acc());

        call_publish_module(root, vec_module_bc(), "Vector");
        call_publish_module(root, event_module_bc(), "Event");

        // we need next block because events are not populated on genesis:
        roll_next_block();

        assert!(Sys::events().is_empty());

        call_execute_script(origin);

        // construct event that should be emitted in the method call directly above
        let expected = RawEvent::Event(
            vec![71, 85, 73, 68],
            1,
            TypeTag::Struct(StructTag {
                address: root_move_addr(),
                module: Identifier::new("Event").unwrap(),
                name: Identifier::new("U64").unwrap(),
                type_params: Vec::with_capacity(0),
            }),
            42u64.to_le_bytes().to_vec(),
        )
        .into();

        // iterate through array of `EventRecord`s
        assert!(Sys::events().iter().any(|rec| rec.event == expected));
    });
}
