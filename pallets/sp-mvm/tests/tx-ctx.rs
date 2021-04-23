use serde::Deserialize;
use frame_support::assert_ok;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;

mod common;
use common::assets::*;
use common::mock::*;
use common::addr::*;
use common::utils::*;

#[derive(Deserialize, Debug, PartialEq)]
struct StoreU64 {
    pub val: u64,
}

fn call_execute_script_tx_block(origin: Origin, tx: UserTx) {
    const GAS_LIMIT: u64 = 1_000_000;
    let txbc = tx.bc().to_vec();

    let result = Mvm::execute(origin, txbc, GAS_LIMIT);
    eprintln!("execute_script result: {:?}", result);
    assert_ok!(result);
}

fn check_stored_value(expected: u64) {
    let expected = StoreU64 { val: expected };

    let tag = StructTag {
        address: origin_move_addr(),
        module: Identifier::new(UserMod::Store.name()).unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };

    check_storage_res(origin_move_addr(), tag, expected);
}

#[test]
fn execute_store_block() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);

        publish_module(root, StdMod::Block);
        publish_module(origin, UserMod::Store);

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        call_execute_script_tx_block(signer, UserTx::StoreSysBlock);
        check_stored_value(EXPECTED);
    });
}

#[test]
fn execute_store_time() {
    new_test_ext().execute_with(|| {
        let root = root_ps_acc();
        let origin = origin_ps_acc();
        let signer = Origin::signed(origin);

        publish_module(root, StdMod::Time);
        publish_module(origin, UserMod::Store);

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        call_execute_script_tx_block(signer, UserTx::StoreSysTime);
        check_stored_value(EXPECTED * TIME_BLOCK_MULTIPLIER);
    });
}
