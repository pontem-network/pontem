use serde::Deserialize;
use frame_system as system;
use frame_support::assert_ok;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_vm::data::*;
use move_vm_runtime::data_cache::RemoteCache;
use sp_mvm::storage::MoveVmStorage;

mod common;
use common::assets::*;
use common::mock::*;
use common::utils::*;

#[derive(Deserialize)]
struct StoreU64 {
    pub val: u64,
}

fn call_publish_module(signer: <Test as system::Config>::AccountId, bc: Vec<u8>, mod_name: &str) {
    const GAS_LIMIT: u64 = 1_000_000;

    let origin = Origin::signed(signer);
    // execute VM for publish module:
    let result = Mvm::publish_module(origin, bc.clone(), GAS_LIMIT);
    eprintln!("publish_module result: {:?}", result);
    assert_ok!(result);

    // check storage:
    let module_id = ModuleId::new(to_move_addr(signer), Identifier::new(mod_name).unwrap());
    let storage = Mvm::move_vm_storage();
    let oracle = MockOracle(None);
    let state = State::new(storage, oracle);
    assert_eq!(bc, state.get_module(&module_id).unwrap().unwrap());
}

fn call_execute_script(origin: Origin) {
    const GAS_LIMIT: u64 = 1_000_000;
    let txbc = UserTx::StoreU64.bc().to_vec();

    // execute VM tx:
    let result = Mvm::execute(origin, txbc, GAS_LIMIT);
    eprintln!("execute_script result: {:?}", result);
    assert_ok!(result);

    // check storage:
    let store = Mvm::move_vm_storage();
    let oracle = MockOracle(None);
    let state = State::new(store, oracle);
    let tag = StructTag {
        address: origin_move_addr(),
        module: Identifier::new(UserMod::Store.name()).unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };
    let blob = state
        .get_resource(&origin_move_addr(), &tag)
        .unwrap()
        .unwrap();
    let store: StoreU64 = bcs::from_bytes(&blob).unwrap();
    assert_eq!(42, store.val);
}

#[test]
fn publish_module() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();
        let module = UserMod::Store;

        call_publish_module(origin, module.bc().to_vec(), module.name());
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);
        let module = UserMod::Store;

        call_publish_module(origin, module.bc().to_vec(), module.name());
        call_execute_script(signer);
    });
}
