#![allow(dead_code)]

const ROOT_PACKAGES: &[&str] = &["Assets"];
const ROOT_PACKAGES_BYTECODE: &[&[u8]] = &[include_bytes!(
    "../assets/root/artifacts/bundles/assets.pac"
)];
const ROOT_PACKAGES_MODULES: &[&[&str]] = &[&["Store", "EventProxy"]];

const USR_PACKAGES: &[&str] = &["Assets"];
const USR_PACKAGES_BYTECODE: &[&[u8]] = &[include_bytes!(
    "../assets/user/artifacts/bundles/assets.pac"
)];
const USR_PACKAGES_MODULES: &[&[&str]] = &[&["Store", "EventProxy"]];

const USER_MODULES: &[&str] = &["Store", "EventProxy"];
const USER_BYTECODE: &[&[u8]] = &[
    include_bytes!("../assets/user/artifacts/modules/2_Store.mv"),
    include_bytes!("../assets/user/artifacts/modules/47_EventProxy.mv"),
];

const ROOT_MODULES: &[&str] = &["Store", "EventProxy"];
const ROOT_BYTECODE: &[&[u8]] = &[
    include_bytes!("../assets/root/artifacts/modules/0_Store.mv"),
    include_bytes!("../assets/root/artifacts/modules/1_EventProxy.mv"),
];

const TX_NAMES: &[&str] = &[
    "store_u64",
    "emit_event",
    "store_system_block",
    "store_system_timestamp",
    "inf_loop",
    "store_native_balance",
    "transfer",
    // "store_native_deposit",
    // "store_native_deposit_reg",
    // "store_native_withdraw",
    // "store_native_withdraw_reg",
    // "get_price_test",
    // "missed_native_balance",
];
const TX_BYTECODE: &[&[u8]] = &[
    include_bytes!("../assets/user/artifacts/transactions/store_u64.mvt"),
    include_bytes!("../assets/user/artifacts/transactions/emit_event.mvt"),
    include_bytes!("../assets/user/artifacts/transactions/store_system_block.mvt"),
    include_bytes!("../assets/user/artifacts/transactions/store_system_timestamp.mvt"),
    include_bytes!("../assets/user/artifacts/transactions/inf_loop.mvt"),
    include_bytes!("../assets/user/artifacts/transactions/store_native_balance.mvt"),
    include_bytes!("../assets/user/artifacts/transactions/transfer.mvt"),
    include_bytes!("../assets/user/artifacts/transactions/multisig_test.mvt"),
    // include_bytes!("../assets/user/artifacts/transactions/store_native_deposit.mvt"),
    // include_bytes!("../assets/user/artifacts/transactions/store_native_deposit_reg.mvt"),
    // include_bytes!("../assets/user/artifacts/transactions/store_native_withdraw.mvt"),
    // include_bytes!("../assets/user/artifacts/transactions/store_native_withdraw_reg.mvt"),
    // include_bytes!("../assets/user/artifacts/transactions/get_price_test.mvt"),
    // include_bytes!("../assets/user/artifacts/transactions/missed_native_balance.mvt"),
];

pub trait BinAsset: Sized + Copy + Into<usize> {
    const NAMES: &'static [&'static str];
    const BYTES: &'static [&'static [u8]];

    fn name(&self) -> &'static str {
        Self::NAMES[(*self).into()]
    }
    fn bc(&self) -> &'static [u8] {
        Self::BYTES[(*self).into()]
    }

    fn all() -> &'static [Self];
}

pub trait BinAssetPackage: BinAsset {
    const MODULES: &'static [&'static [&'static str]];

    fn modules(&self) -> &[&'static str] {
        Self::MODULES[(*self).into()]
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum UserMod {
    Store = 0,
    EventProxy = 1,
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum RootMod {
    Store = 0,
    EventProxy = 1,
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum RootPackages {
    Assets = 0,
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum UsrPackages {
    Assets = 0,
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum UserTx {
    StoreU64 = 0,
    EmitEvent = 1,
    StoreSysBlock = 2,
    StoreSysTime = 3,
    InfLoop = 4,
    StoreGetBalance = 5,
    Transfer = 6,
    MultisigTest = 7,
    //StoreNativeDeposit = 6,
    //StoreNativeDepositReg = 7,
    //StoreNativeWithdraw = 8,
    //StoreNativeWithdrawReg = 9,
    //GetPriceTest = 10,
    //MissedNativeBalance = 11,
}

impl Into<usize> for UserMod {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for RootMod {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for RootPackages {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for UsrPackages {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for UserTx {
    fn into(self) -> usize {
        self as usize
    }
}

impl BinAsset for UserMod {
    const NAMES: &'static [&'static str] = USER_MODULES;
    const BYTES: &'static [&'static [u8]] = USER_BYTECODE;

    fn all() -> &'static [Self] {
        &[Self::Store, Self::EventProxy]
    }
}

impl BinAsset for RootMod {
    const NAMES: &'static [&'static str] = ROOT_MODULES;
    const BYTES: &'static [&'static [u8]] = ROOT_BYTECODE;

    fn all() -> &'static [Self] {
        &[Self::Store, Self::EventProxy]
    }
}

impl BinAsset for UserTx {
    const NAMES: &'static [&'static str] = TX_NAMES;
    const BYTES: &'static [&'static [u8]] = TX_BYTECODE;

    fn all() -> &'static [Self] {
        &[
            Self::StoreU64,
            Self::EmitEvent,
            Self::StoreSysBlock,
            Self::StoreSysTime,
            Self::InfLoop,
            Self::StoreGetBalance,
            Self::Transfer,
        ]
    }
}

impl BinAsset for RootPackages {
    const NAMES: &'static [&'static str] = ROOT_PACKAGES;
    const BYTES: &'static [&'static [u8]] = ROOT_PACKAGES_BYTECODE;

    fn all() -> &'static [Self] {
        &[Self::Assets]
    }
}

impl BinAssetPackage for RootPackages {
    const MODULES: &'static [&'static [&'static str]] = ROOT_PACKAGES_MODULES;
}

impl BinAsset for UsrPackages {
    const NAMES: &'static [&'static str] = USR_PACKAGES;
    const BYTES: &'static [&'static [u8]] = USR_PACKAGES_BYTECODE;

    fn all() -> &'static [Self] {
        &[Self::Assets]
    }
}

impl BinAssetPackage for UsrPackages {
    const MODULES: &'static [&'static [&'static str]] = USR_PACKAGES_MODULES;
}
