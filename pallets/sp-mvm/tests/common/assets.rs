#![allow(dead_code)]

use once_cell::sync::OnceCell;
use std::path::Path;

#[derive(Clone)]
pub struct Asset {
    name: &'static str,
    path: &'static str,
    bytes: OnceCell<Vec<u8>>,
}

impl Asset {
    pub const fn new(name: &'static str, path: &'static str) -> Self {
        Self {
            name,
            path,
            bytes: OnceCell::new(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
            .get_or_init(|| {
                let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
                let path = Path::new(dir.as_str()).join(self.path);
                std::fs::read(&path)
                    .unwrap_or_else(|_| panic!("Failed to load test asset: {:?}", path.display()))
            })
            .as_slice()
    }
}

pub struct Package {
    modules: &'static [&'static str],
    package: Asset,
}

impl Package {
    pub const fn new(modules: &'static [&'static str], package: Asset) -> Self {
        Self { modules, package }
    }
    pub fn modules(&self) -> &'static [&'static str] {
        self.modules
    }
    pub fn bytes(&self) -> &[u8] {
        self.package.bytes()
    }
}

pub static ROOT_PACKAGE: Package = Package::new(
    &["Store", "EventProxy"],
    Asset::new("", "tests/assets/root/artifacts/bundles/assets.pac"),
);
pub static USER_PACKAGE: Package = Package::new(
    &["Store", "EventProxy"],
    Asset::new("", "tests/assets/user/artifacts/bundles/assets.pac"),
);

pub mod modules {
    pub mod root {
        use super::super::Asset;
        pub static STORE: Asset =
            Asset::new("Store", "tests/assets/root/artifacts/modules/0_Store.mv");
        pub static EVENT_PROXY: Asset = Asset::new(
            "EventProxy",
            "tests/assets/root/artifacts/modules/1_EventProxy.mv",
        );
    }

    pub mod user {
        use super::super::Asset;
        pub static STORE: Asset =
            Asset::new("Store", "tests/assets/user/artifacts/modules/2_Store.mv");
        pub static EVENT_PROXY: Asset = Asset::new(
            "EventProxy",
            "tests/assets/user/artifacts/modules/47_EventProxy.mv",
        );
    }
}

pub mod transactions {
    use super::Asset;
    pub static STORE_U64: Asset = Asset::new(
        "store_u64",
        "tests/assets/user/artifacts/transactions/store_u64.mvt",
    );
    pub static EMIT_EVENT: Asset = Asset::new(
        "emit_event",
        "tests/assets/user/artifacts/transactions/emit_event.mvt",
    );
    pub static STORE_SYSTEM_BLOCK: Asset = Asset::new(
        "store_system_block",
        "tests/assets/user/artifacts/transactions/store_system_block.mvt",
    );
    pub static STORE_SYSTEM_TIMESTAMP: Asset = Asset::new(
        "store_system_timestamp",
        "tests/assets/user/artifacts/transactions/store_system_timestamp.mvt",
    );
    pub static INF_LOOP: Asset = Asset::new(
        "inf_loop",
        "tests/assets/user/artifacts/transactions/inf_loop.mvt",
    );
    pub static STORE_NATIVE_BALANCE: Asset = Asset::new(
        "store_native_balance",
        "tests/assets/user/artifacts/transactions/store_native_balance.mvt",
    );
    pub static STORE_TOKEN_BALANCE: Asset = Asset::new(
        "store_token_balance",
        "tests/assets/user/artifacts/transactions/store_token_balance.mvt",
    );
    pub static TRANSFER: Asset = Asset::new(
        "transfer",
        "tests/assets/user/artifacts/transactions/transfer.mvt",
    );
    pub static TRANSFER_TOKEN: Asset = Asset::new(
        "transfer_token",
        "tests/assets/user/artifacts/transactions/transfer_token.mvt",
    );
    pub static MULTISIG_TEST: Asset = Asset::new(
        "multisig_test",
        "tests/assets/user/artifacts/transactions/multisig_test.mvt",
    );
}
