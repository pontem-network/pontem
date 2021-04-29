use std::convert::TryInto;
use codec::Encode;
use frame_support::assert_ok;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;

use sp_mvm::types::MoveStructTag;
use sp_mvm::types::MoveTypeTag;
use sp_mvm::Event;

mod common;
use common::assets::*;
use common::mock::*;
use common::addr::*;
use common::utils;

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
        utils::publish_module(root, StdMod::Event);
    });
}

#[test]
#[ignore = "FIXME: Origin::root() produces BadOrigin because we should build move with `to_move_addr(Origin::root())`."]
/// publish std modules as root
fn publish_module_as_root() {
    new_test_ext().execute_with(|| {
        let event = StdMod::Event;
        let proxy = UserMod::EventProxy;

        utils::publish_module_raw_with_origin_unchecked(Origin::root(), event.bc().to_vec());
        utils::check_storage_mod_raw_with_addr(ROOT_ADDR, event.bc().to_vec(), event.name());

        utils::publish_module_raw_with_origin_unchecked(Origin::root(), proxy.bc().to_vec());
        utils::check_storage_mod_raw_with_addr(ROOT_ADDR, proxy.bc().to_vec(), proxy.name());
    });
}

#[test]
fn execute_script() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = origin_ps_acc();
        let event = StdMod::Event;
        let proxy = UserMod::EventProxy;

        utils::publish_module(root, event);
        utils::publish_module(origin, proxy);

        // we need next block because events are not populated on genesis:
        roll_next_block();

        assert!(Sys::events().is_empty());

        call_execute_script(Origin::signed(origin));

        // construct event: that should be emitted in the method call directly above
        let tt = MoveTypeTag::Struct(MoveStructTag::new(
            origin,
            Identifier::new(proxy.name()).unwrap(),
            Identifier::new("U64").unwrap(),
            Vec::with_capacity(0),
        ));

        let expected = vec![
            // one for user::Proxy -> std::Event (`Event::emit`)
            Event::Event(origin, tt.encode(), 42u64.to_le_bytes().to_vec(), None).into(),
            // and one for user::Proxy -> std::Event (`EventProxy::emit_event`)
            Event::Event(
                origin,
                tt.encode(),
                42u64.to_le_bytes().to_vec(),
                Some(
                    ModuleId::new(to_move_addr(origin), Identifier::new(proxy.name()).unwrap())
                        .try_into()
                        .unwrap(),
                ),
            )
            .into(),
        ];

        expected.into_iter().for_each(|expected| {
            // iterate through array of `EventRecord`s
            assert!(Sys::events().iter().any(|rec| {
                // TODO: compare only required fields
                rec.event == expected
            }))
        })
    });
}

#[test]
#[ignore = "FIXME: Origin::root() produces BadOrigin because we should build move with `to_move_addr(Origin::root())`."]
/// publish package as root
fn publish_package_as_root() {
    new_test_ext().execute_with(|| {
        utils::publish_std();

        const GAS_LIMIT: u64 = 1_000_000;
        let package = Packages::Assets;
        let root = root_ps_acc();

        utils::publish_package(root, package, GAS_LIMIT);
    });
}

#[test]
/// publish package as origin
fn publish_package_as_origin() {
    new_test_ext().execute_with(|| {
        utils::publish_std();

        const GAS_LIMIT: u64 = 1_000_000;
        let package = Packages::Assets;
        let origin = origin_ps_acc();

        utils::publish_package(origin, package, GAS_LIMIT);
    });
}
