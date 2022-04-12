use crate::tests::mock::{RuntimeBuilder, Accounts, Mvm, Origin, run_to_block, TIME_BLOCK_MULTIPLIER};
use frame_support::assert_ok;
use move_core_types::{
    language_storage::StructTag, identifier::Identifier, account_address::AccountAddress,
    resolver::ResourceResolver,
};
use sp_mvm::storage::MoveVmStorage;
use move_vm::io::state::State;

pub mod modules {
    use assets::Asset;
    pub static STORE: Asset = Asset::new(
        "Store",
        "src/tests/assets/user/build/assets/bytecode_modules/Store.mv",
    );
}

pub mod transactions {
    use assets::Asset;
    pub static STORE_SYSTEM_BLOCK: Asset = Asset::new(
        "store_system_block",
        "src/tests/assets/user/build/assets/transaction/store_system_block.mvt",
    );
    pub static STORE_SYSTEM_TIMESTAMP: Asset = Asset::new(
        "store_system_timestamp",
        "src/tests/assets/user/build/assets/transaction/store_system_timestamp.mvt",
    );
}

const GAS_LIMIT: u64 = 1_000_000;

/// Check stored value (u64) inside storage.
fn check_stored_value(expected: u64) {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct StoreU64 {
        pub val: u64,
    }

    let expected = StoreU64 { val: expected };

    let tag = StructTag {
        address: Accounts::BOB.addr(),
        module: Identifier::new(modules::STORE.name()).unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };

    check_storage_res(Accounts::BOB.addr(), tag, expected);
}

/// Check resource value inside storage.
pub fn check_storage_res<T>(owner: AccountAddress, ty: StructTag, expected: T)
where
    T: for<'de> serde::Deserialize<'de>,
    T: std::cmp::PartialEq + std::fmt::Debug,
{
    let storage = Mvm::move_vm_storage();
    let state = State::new(storage);
    let blob = state
        .get_resource(&owner, &ty)
        .expect("VM state read storage (resource)")
        .expect(&format!("Resource '{}' should exist", ty));

    let tt_str = format!("{}", ty);
    println!("checking stored resource '{}'", tt_str);
    let stored: T =
        bcs::from_bytes(&blob).expect(&format!("Resource '{}' should exists", tt_str));
    assert_eq!(expected, stored);
}

#[test]
/// Execute storing of block height inside module by calling script.
fn execute_store_block() {
    RuntimeBuilder::new().build().execute_with(|| {
        // Publish STORE module.
        assert_ok!(Mvm::publish_module(
            Origin::signed(Accounts::BOB.account()),
            modules::STORE.bytes().to_vec(),
            GAS_LIMIT
        ));

        const EXPECTED: u32 = 3;
        run_to_block(EXPECTED);
        assert_ok!(Mvm::execute(
            Origin::signed(Accounts::BOB.account()),
            transactions::STORE_SYSTEM_BLOCK.bytes().to_vec(),
            GAS_LIMIT
        ));
        check_stored_value(EXPECTED.into());
    });
}

#[test]
/// Execute storing of timestamp inside module by calling script.
fn execute_store_time() {
    RuntimeBuilder::new().build().execute_with(|| {
        // Publish STORE module.
        assert_ok!(Mvm::publish_module(
            Origin::signed(Accounts::BOB.account()),
            modules::STORE.bytes().to_vec(),
            GAS_LIMIT
        ));

        const EXPECTED: u32 = 3;
        run_to_block(EXPECTED);
        assert_ok!(Mvm::execute(
            Origin::signed(Accounts::BOB.account()),
            transactions::STORE_SYSTEM_TIMESTAMP.bytes().to_vec(),
            GAS_LIMIT
        ));
        check_stored_value(EXPECTED as u64 * TIME_BLOCK_MULTIPLIER);
    });
}
