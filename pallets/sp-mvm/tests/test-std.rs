use codec::Encode;
use frame_system as system;
use frame_support::assert_ok;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_vm_runtime::data_cache::RemoteCache;
use move_core_types::language_storage::TypeTag;
use move_core_types::language_storage::StructTag;
use move_vm::data::*;

use sp_mvm::storage::MoveVmStorage;
use sp_mvm::event::MoveRawEvent as RawEvent;

mod common;
use common::assets::*;
use common::mock::*;
use common::utils::*;

fn call_publish_module_with_origin(origin: Origin, bc: Vec<u8>) {
    const GAS_LIMIT: u64 = 1_000_000;

    // execute VM for publish module:
    let result = Mvm::publish_module(origin, bc, GAS_LIMIT);
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
    let oracle = MockOracle(None);
    let state = State::new(storage, oracle);
    assert_eq!(bc, state.get_module(&module_id).unwrap().unwrap());
}

fn check_storage_with_addr(signer: AccountAddress, bc: Vec<u8>, mod_name: &str) {
    // check storage:
    let module_id = ModuleId::new(signer, Identifier::new(mod_name).unwrap());
    let storage = Mvm::move_vm_storage();
    let oracle = MockOracle(None);
    let state = State::new(storage, oracle);
    assert_eq!(bc, state.get_module(&module_id).unwrap().unwrap());
}

fn call_execute_script(origin: Origin) {
    const GAS_LIMIT: u64 = 1_000_000;

    // execute VM tx:
    let result = Mvm::execute(origin, UserTx::EmitEvent.bc().to_vec(), GAS_LIMIT);
    eprintln!("tx result: {:?}", result);
    assert_ok!(result);
}

#[test]
/// publish modules personally as root
fn publish_module() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let module = StdMod::Event;
        call_publish_module(root, module.bc().to_vec(), module.name());
    });
}

#[test]
#[ignore = "FIXME: Origin::root() produces BadOrigin because we should build move with `to_move_addr(Origin::root())`."]
/// publish std modules as root
fn publish_module_as_root() {
    new_test_ext().execute_with(|| {
        let event = StdMod::Event;
        let proxy = UserMod::EventProxy;

        call_publish_module_with_origin(Origin::root(), event.bc().to_vec());
        check_storage_with_addr(CORE_CODE_ADDRESS, event.bc().to_vec(), event.name());

        call_publish_module_with_origin(Origin::root(), proxy.bc().to_vec());
        check_storage_with_addr(CORE_CODE_ADDRESS, proxy.bc().to_vec(), proxy.name());
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = origin_ps_acc();
        let event = StdMod::Event;
        let proxy = UserMod::EventProxy;

        call_publish_module(root, event.bc().to_vec(), event.name());
        call_publish_module(origin, proxy.bc().to_vec(), proxy.name());

        // we need next block because events are not populated on genesis:
        roll_next_block();

        assert!(Sys::events().is_empty());

        call_execute_script(Origin::signed(origin));

        // construct event: that should be emitted in the method call directly above
        let tt = TypeTag::Struct(StructTag {
            address: to_move_addr(origin),
            module: Identifier::new(proxy.name()).unwrap(),
            name: Identifier::new("U64").unwrap(),
            type_params: Vec::with_capacity(0),
        });
        let expected = vec![
            // one for user::Proxy -> std::Event (`Event::emit`)
            RawEvent::Event(
                to_move_addr(origin),
                tt.encode(),
                42u64.to_le_bytes().to_vec(),
                None,
            )
            .into(),
            // and one for user::Proxy -> std::Event (`EventProxy::emit_event`)
            RawEvent::Event(
                to_move_addr(origin),
                tt.encode(),
                42u64.to_le_bytes().to_vec(),
                Some(ModuleId::new(
                    to_move_addr(origin),
                    Identifier::new(proxy.name()).unwrap(),
                )),
            )
            .into(),
        ];

        expected.into_iter().for_each(|expected| {
            // iterate through array of `EventRecord`s
            assert!(Sys::events().iter().any(|rec| rec.event == expected))
        })
    });
}
