const STD_MODULES: &[&str] = &["Block", "Event", "Time"];
const STD_BYTECODE: &[&[u8]] = &[
    include_bytes!("../assets/target/modules/0_Block.mv"),
    include_bytes!("../assets/target/modules/1_Event.mv"),
    include_bytes!("../assets/target/modules/2_Time.mv"),
];

const USER_MODULES: &[&str] = &["Store", "EventProxy"];
const USER_BYTECODE: &[&[u8]] = &[
    include_bytes!("../assets/target/modules/3_Store.mv"),
    include_bytes!("../assets/target/modules/4_EventProxy.mv"),
];

const TX_NAMES: &[&str] = &[
    "store_u64",
    "emit_event",
    "store_system_block",
    "store_system_timestamp",
    "inf_loop",
];
const TX_BYTECODE: &[&[u8]] = &[
    include_bytes!("../assets/target/transactions/store_u64.mvt"),
    include_bytes!("../assets/target/transactions/emit_event.mvt"),
    include_bytes!("../assets/target/transactions/store_system_block.mvt"),
    include_bytes!("../assets/target/transactions/store_system_timestamp.mvt"),
    include_bytes!("../assets/target/transactions/inf_loop.mvt"),
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

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum StdMod {
    Block = 0,
    Event = 1,
    Time = 2,
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum UserMod {
    Store = 0,
    EventProxy = 1,
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum UserTx {
    StoreU64 = 0,
    EmitEvent = 1,
    StoreSysBlock = 2,
    StoreSysTime = 3,
    InfLoop = 4,
}

impl Into<usize> for StdMod {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for UserMod {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for UserTx {
    fn into(self) -> usize {
        self as usize
    }
}

impl BinAsset for StdMod {
    const NAMES: &'static [&'static str] = STD_MODULES;
    const BYTES: &'static [&'static [u8]] = STD_BYTECODE;

    fn all() -> &'static [Self] {
        &[Self::Block, Self::Event, Self::Time]
    }
}

impl BinAsset for UserMod {
    const NAMES: &'static [&'static str] = USER_MODULES;
    const BYTES: &'static [&'static [u8]] = USER_BYTECODE;

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
        ]
    }
}
