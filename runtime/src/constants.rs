pub mod currency {
    use crate::primitives::Balance;

    // Currencies constants.
    // Decimals
    pub const DECIMALS: u32 = 18;

    // 1 PONT.
    pub const PONT: Balance = u128::pow(10, DECIMALS);
}

pub mod time {
    use crate::primitives::{Moment, BlockNumber};

    /// This determines the average expected block time that we are targeting.
    /// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
    /// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
    /// up by `pallet_aura` to implement `fn slot_duration()`.
    ///
    /// Change this to adjust the block time.
    pub const MILLISECS_PER_BLOCK: u64 = 6000;

    // Network slot duration allocated for block producing.
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

    // How much seconds required to generate block.
    pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

    // 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

    // Time is measured by number of blocks.
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
}
