/// Currencies constants.
use primitives::{Balance, currency::CurrencyId};

/// Units.
pub const PONT: Balance = u64::pow(10, CurrencyId::PONT.decimals() as _);
pub const UNIT: Balance = PONT;
pub const MILLIUNIT: Balance = UNIT / 1_000;
pub const MICROUNIT: Balance = MILLIUNIT / 1_000;

/// Essential Deposits.
pub const PONT_EXISTENTIAL_DEPOSIT: Balance = 100;
pub const KSM_EXISTENTIAL_DEPOSIT: Balance = 100000;
