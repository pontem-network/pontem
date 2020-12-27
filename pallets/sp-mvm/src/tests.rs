use crate::{mock::*};
use frame_support::assert_ok;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_core_types::value::MoveTypeLayout;
use move_vm::data::*;
use move_vm_types::values::Value;
use move_vm_runtime::data_cache::RemoteCache;
use move_vm::types::*;
use serde::Deserialize;

fn store_module() -> ModuleTx {
    ModuleTx::new(store_module_bc(), CORE_CODE_ADDRESS)
}

fn store_module_bc() -> Vec<u8> {
    include_bytes!("../tests/assets/Store.mv").to_vec()
}

fn script(args: u64) -> ScriptTx {
    ScriptTx::new(
        script_bc(),
        vec![Value::u64(args)],
        vec![],
        vec![CORE_CODE_ADDRESS],
    )
    .unwrap()
}

fn script_bc() -> Vec<u8> {
    include_bytes!("../tests/assets/Script.mv").to_vec()
}

fn gas() -> Gas {
    Gas::new(10_000, 1).unwrap()
}

#[derive(Deserialize)]
struct StoreU64 {
    pub val: u64,
}

fn call_publish_module(origin: Origin) {
    let result = Mvm::publish_module(origin, store_module_bc());
    eprintln!("result: {:?}", result);
    assert_ok!(result);

    let store_module_id = ModuleId::new(CORE_CODE_ADDRESS, Identifier::new("Store").unwrap());
    let store = Mvm::get_vm_storage();
    let state = State::new(store);
    assert_eq!(
        store_module_bc(),
        state.get_module(&store_module_id).unwrap().unwrap(),
    );
}

fn call_execute_script(origin: Origin) {
    const TEST_VALUE: u64 = 13;

    let args = Value::u64(TEST_VALUE)
        .simple_serialize(&MoveTypeLayout::U64)
        .unwrap();

    let result = Mvm::execute(origin, script_bc(), args);
    eprintln!("result: {:?}", result);
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

#[test]
fn publish_module() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(CORE_CODE_ADDRESS.to_u8().last().unwrap().to_owned());
        call_publish_module(origin)
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(CORE_CODE_ADDRESS.to_u8().last().unwrap().to_owned());
        call_publish_module(origin.clone());
        call_execute_script(origin);
    });
}

// #[test]
// fn correct_error_for_none_value() {
//     new_test_ext().execute_with(|| {
//         // Ensure the expected error is thrown when no value is present.
//         assert_noop!(
//             Mvm::cause_error(Origin::signed(1)),
//             Error::<Test>::NoneValue
//         );
//     });
// }



// TODO: TEST ACC CONVERSION
// Secret Key URI `//Alice` is account:
//   Secret seed:      0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
//   Public key (hex): 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
//   Account ID:       0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
//   SS58 Address:     5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
// [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125]
