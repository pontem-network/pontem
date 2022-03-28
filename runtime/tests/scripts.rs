mod common;
use serde::Deserialize;
use move_core_types::{identifier::Identifier, language_storage::StructTag};
use common::mocks::{AccountId, Mvm, roll_next_block, RuntimeBuilder, Origin, TIME_BLOCK_MULTIPLIER};
use common::addr::{bob_public_key, to_move_addr, origin_move_addr};
use common::assets::{Asset, modules, transactions};
use common::utils::{self, check_storage_module, DEFAULT_GAS_LIMIT};
use frame_support::dispatch::DispatchResultWithPostInfo;

/// Check stored value (u64) inside storage.
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

    utils::check_storage_res(origin_move_addr(), tag, expected);
}

/// Publish module with storage check.
pub fn publish_module(
    signer: AccountId,
    module: &Asset,
    gas_limit: Option<u64>,
) -> DispatchResultWithPostInfo {
    let result = Mvm::publish_module(
        Origin::signed(signer),
        module.bytes().to_vec(),
        gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
    )?;
    check_storage_module(to_move_addr(signer), module.bytes().to_vec(), module.name());
    Ok(result)
}

#[test]
/// Execute storing of block height inside module by calling script.
fn execute_store_block() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();

        utils::publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        utils::execute_tx(origin, &transactions::STORE_SYSTEM_BLOCK, None).unwrap();
        check_stored_value(EXPECTED);
    });
}

#[test]
/// Execute storing of timestamp inside module by calling script.
fn execute_store_time() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();

        utils::publish_module(origin, &modules::user::STORE, None).unwrap();

        const EXPECTED: u64 = 3;
        for _ in 0..EXPECTED {
            roll_next_block();
        }
        utils::execute_tx(origin, &transactions::STORE_SYSTEM_TIMESTAMP, None).unwrap();
        check_stored_value(EXPECTED * TIME_BLOCK_MULTIPLIER);
    });
}
