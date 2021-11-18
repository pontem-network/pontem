/// Currencies constants.
use primitives::Balance;

/// Decimals.
pub const DECIMALS: u32 = 10;

/// Units.
pub const PONT: Balance = u64::pow(10, DECIMALS);
pub const UNIT: Balance = PONT;
pub const MILLIUNIT: Balance = UNIT / 1_000;
pub const MICROUNIT: Balance = MILLIUNIT / 1_000;

/// Essential Deposits.
pub const PONT_EXISTENTIAL_DEPOSIT: Balance = 100;
pub const KSM_EXISTENTIAL_DEPOSIT: Balance = 100000;
