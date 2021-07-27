use move_core_types::identifier::Identifier;

use move_core_types::language_storage::StructTag;
use move_core_types::language_storage::TypeTag;
use sp_mvm::Event;

mod common;
use common::assets::*;
use common::mock::*;
use common::addr::*;
use common::utils;

#[test]
/// publish modules personally as root
fn publish_module() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        utils::publish_module(root, RootMod::EventProxy, None).unwrap();
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();

        utils::publish_module(origin, UserMod::EventProxy, None).unwrap();

        // we need next block because events are not populated on genesis:
        roll_next_block();

        assert!(Sys::events().is_empty());

        utils::execute_tx(origin, UserTx::EmitEvent, None).unwrap();

        // construct event: that should be emitted in the method call directly above
        let tt = TypeTag::Struct(StructTag {
            address: to_move_addr(origin),
            module: Identifier::new("EventProxy").unwrap(),
            name: Identifier::new("U64").unwrap(),
            type_params: Vec::with_capacity(0),
        })
        .to_string();
        let tt = tt.as_bytes();

        // guid is sequence number (8 bytes) followed by account address
        let mut guid = vec![0; 8];
        guid.extend(&origin.0);

        let expected = Event::Event(guid, tt.to_vec(), 42u64.to_le_bytes().to_vec()).into();

        // iterate through array of `EventRecord`s
        assert!(Sys::events().iter().any(|rec| { rec.event == expected }))
    });
}

#[test]
/// publish package as root
fn publish_package_as_root() {
    new_test_ext().execute_with(|| {
        let package = RootPackages::Assets;
        let root = root_ps_acc();

        utils::publish_package(root, package, None).unwrap();
    });
}

#[test]
/// publish package as origin
fn publish_package_as_origin() {
    new_test_ext().execute_with(|| {
        let package = UsrPackages::Assets;
        let origin = origin_ps_acc();

        utils::publish_package(origin, package, None).unwrap();
    });
}

// TODO: publish std modules as root
// call `utils::publish_module_raw_with_origin_unchecked`
// with `Origin::root()`
// and check there is mod exists for ROOT_ADDR
// NOTE: Origin::root() produces BadOrigin because we should build move with `to_move_addr(Origin::root())`.
