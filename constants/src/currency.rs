/// Currencies constants.
use primitives::{Balance, currency::CurrencyId};

/// Decimals.
pub const DECIMALS: u32 = 10;

/// Units.
pub const PONT: Balance = u64::pow(10, DECIMALS);
pub const UNIT: Balance = PONT;
pub const MILLIUNIT: Balance = UNIT / 1_000;
pub const MICROUNIT: Balance = MILLIUNIT / 1_000;

/// Get existential deposit for currency, in Pontem runtime.
pub fn get_exist_deposit(currency_id: &CurrencyId) -> Balance {
    match currency_id {
        CurrencyId::PONT => 100,
        _ => 100000,
    }
}

#[test]
/// Test `get_exist_deposit` func.
fn test_get_exist_deposit() {
    assert_eq!(get_exist_deposit(&CurrencyId::PONT), 100);
    assert_eq!(get_exist_deposit(&CurrencyId::KSM), 100000);
    assert_eq!(get_exist_deposit(&CurrencyId::KAR), 100000);
    assert_eq!(get_exist_deposit(&CurrencyId::KUSD), 100000);
    assert_eq!(get_exist_deposit(&CurrencyId::LKSM), 100000);
}
