use serde::Deserialize;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;

mod common;
use common::assets::{modules, transactions};
use common::mock::*;
use common::addr::*;
use common::utils::*;

fn check_stored_value(expected: u64) {
    #[derive(Deserialize, Debug, PartialEq)]
    struct StoreU64 {
        pub val: u64,
    }

    let expected = StoreU64 { val: expected };

    let tag = StructTag {
        address: origin_move_addr(),
        module: Identifier::new(modules::user::STORE.name()).unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };

    check_storage_res(origin_move_addr(), tag, expected);
}

#[test]
fn execute_store_block() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();

        publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        execute_tx(origin, &transactions::STORE_SYSTEM_BLOCK, None).unwrap();
        check_stored_value(EXPECTED);
    });
}

#[test]
fn execute_store_time() {
    new_test_ext().execute_with(|| {
        let origin = origin_ps_acc();

        publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        execute_tx(origin, &transactions::STORE_SYSTEM_TIMESTAMP, None).unwrap();
        check_stored_value(EXPECTED * TIME_BLOCK_MULTIPLIER);
    });
}
