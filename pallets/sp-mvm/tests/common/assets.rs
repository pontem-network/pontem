#![allow(dead_code)]

pub use assets::*;

pub static ROOT_PACKAGE: Package = Package::new(
    &["EventProxy", "Store"],
    Asset::new("", "tests/assets/root/build/assets/bundles/assets.pac"),
);
pub static USER_PACKAGE: Package = Package::new(
    &["Bank", "EventProxy", "Store"],
    Asset::new("", "tests/assets/user/build/assets/bundles/assets.pac"),
);

pub mod modules {
    pub mod root {
        use super::super::Asset;
        pub static STORE: Asset = Asset::new(
            "Store",
            "tests/assets/root/build/assets/bytecode_modules/Store.mv",
        );
        pub static EVENT_PROXY: Asset = Asset::new(
            "EventProxy",
            "tests/assets/root/build/assets/bytecode_modules/EventProxy.mv",
        );
    }

    pub mod user {
        use super::super::Asset;
        pub static STORE: Asset = Asset::new(
            "Store",
            "tests/assets/user/build/assets//bytecode_modules/Store.mv",
        );
        pub static EVENT_PROXY: Asset = Asset::new(
            "EventProxy",
            "tests/assets/user/build/assets//bytecode_modules/EventProxy.mv",
        );
        pub static BANK: Asset = Asset::new(
            "Bank",
            "tests/assets/user/build/assets/bytecode_modules/Bank.mv",
        );
    }
}

pub mod transactions {
    use super::Asset;
    pub static STORE_U64: Asset = Asset::new(
        "store_u64",
        "tests/assets/user/build/assets/transaction/store_u64.mvt",
    );
    pub static EMIT_EVENT: Asset = Asset::new(
        "emit_event",
        "tests/assets/user/build/assets/transaction/emit_event.mvt",
    );
    pub static STORE_SYSTEM_BLOCK: Asset = Asset::new(
        "store_system_block",
        "tests/assets/user/build/assets/transaction/store_system_block.mvt",
    );
    pub static STORE_SYSTEM_TIMESTAMP: Asset = Asset::new(
        "store_system_timestamp",
        "tests/assets/user/build/assets/transaction/store_system_timestamp.mvt",
    );
    pub static INF_LOOP: Asset = Asset::new(
        "inf_loop",
        "tests/assets/user/build/assets/transaction/inf_loop.mvt",
    );
    pub static STORE_NATIVE_BALANCE: Asset = Asset::new(
        "store_native_balance",
        "tests/assets/user/build/assets/transaction/store_native_balance.mvt",
    );
    pub static STORE_TOKEN_BALANCE: Asset = Asset::new(
        "store_token_balance",
        "tests/assets/user/build/assets/transaction/store_token_balance.mvt",
    );
    pub static TRANSFER: Asset = Asset::new(
        "transfer",
        "tests/assets/user/build/assets/transaction/transfer.mvt",
    );
    pub static TRANSFER_TOKEN: Asset = Asset::new(
        "transfer_token",
        "tests/assets/user/build/assets/transaction/transfer_token.mvt",
    );
    pub static MULTISIG_TEST: Asset = Asset::new(
        "multisig_test",
        "tests/assets/user/build/assets/transaction/multisig_test.mvt",
    );
    pub static DEPOSIT_BANK_PONT: Asset = Asset::new(
        "deposit_bank_pont",
        "tests/assets/user/build/assets/transaction/deposit_bank_pont.mvt",
    );
    pub static DEPOSIT_BANK_KSM: Asset = Asset::new(
        "deposit_bank_ksm",
        "tests/assets/user/build/assets/transaction/deposit_bank_ksm.mvt",
    );
    pub static AS_ROOT: Asset = Asset::new(
        "as_root",
        "tests/assets/user/build/assets/transaction/as_root.mvt",
    );
}
