// Currency constants.
pub mod currency {
    use crate::primitives::Balance;

    // Currencies constants.
    // Decimals.
    pub const DECIMALS: u32 = 10;

    // 1 PONT.
    pub const PONT: Balance = u64::pow(10, DECIMALS);

    // Essential Deposits.
    pub const PONT_EXISTENTIAL_DEPOSIT: Balance = 100;
    pub const KSM_EXISTENTIAL_DEPOSIT: Balance = 100000;
}

// Time related constants.
pub mod time {
    use crate::primitives::{BlockNumber};

    /// This determines the average expected block time that we are targeting.
    /// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
    /// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
    /// up by `pallet_aura` to implement `fn slot_duration()`.
    ///
    /// Change this to adjust the block time.
    pub const MILLISECS_PER_BLOCK: u64 = 12000;

    // Network slot duration allocated for block producing.
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

    // Time is measured by number of blocks.

    // 10 blocks per minute.
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);

    // 600 blocks per hour.
    pub const HOURS: BlockNumber = MINUTES * 60;

    // 14400 blocks per day.
    pub const DAYS: BlockNumber = HOURS * 24;
}
