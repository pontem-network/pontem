/// Supported currencies.
/// TODO: would be good to replace with Acala currencies: https://github.com/AcalaNetwork/Acala/blob/master/primitives/src/currency.rs.
/// Or implement something similar (without EVM/DEX, etc.)
use sp_core::RuntimeDebug;
use sp_std::convert::TryFrom;
use sp_std::vec::Vec;
use sp_std::cmp::PartialEq;
use sp_std::default::Default;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// Currencies id.
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
    // Relaychain's currency.
    KSM,
    // Our native currency.
    PONT,
    // Karura token.
    KAR,
    // Karura USD.
    KUSD,
    // Liquid KSM.
    LKSM,
}

/// Implement currencies.
impl CurrencyId {
    pub fn decimals(&self) -> u8 {
        match self {
            Self::KSM => 12,
            Self::PONT => 10,
            Self::KAR => 12,
            Self::KUSD => 12,
            Self::LKSM => 12,
        }
    }

    pub fn symbol(&self) -> Vec<u8> {
        match self {
            Self::KSM => b"KSM".to_vec(),
            Self::PONT => b"PONT".to_vec(),
            Self::KAR => b"KAR".to_vec(),
            Self::KUSD => b"KUSD".to_vec(),
            Self::LKSM => b"LKSM".to_vec(),
        }
    }
}

/// Try from.
impl TryFrom<Vec<u8>> for CurrencyId {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match value {
            v if v == CurrencyId::PONT.symbol() => Ok(CurrencyId::PONT),
            v if v == CurrencyId::KSM.symbol() => Ok(CurrencyId::KSM),
            v if v == CurrencyId::KAR.symbol() => Ok(CurrencyId::KAR),
            v if v == CurrencyId::KUSD.symbol() => Ok(CurrencyId::KUSD),
            v if v == CurrencyId::LKSM.symbol() => Ok(CurrencyId::LKSM),
            _ => Err(()),
        }
    }
}

impl Default for CurrencyId {
    fn default() -> Self {
        // PONT should be default currency.
        CurrencyId::PONT
    }
}

#[test]
/// Test default currency.
fn test_default() {
    assert_eq!(CurrencyId::default(), CurrencyId::PONT);
}

#[test]
/// Test currencies decimals.
fn test_decimals() {
    assert_eq!(CurrencyId::PONT.decimals(), 10);
    assert_eq!(CurrencyId::KSM.decimals(), 12);
    assert_eq!(CurrencyId::KAR.decimals(), 12);
    assert_eq!(CurrencyId::KUSD.decimals(), 12);
    assert_eq!(CurrencyId::LKSM.decimals(), 12);
}

#[test]
/// Test currencies symbols.
fn test_symbols() {
    assert_eq!(CurrencyId::PONT.symbol(), b"PONT");
    assert_eq!(CurrencyId::KSM.symbol(), b"KSM");
    assert_eq!(CurrencyId::KAR.symbol(), b"KAR");
    assert_eq!(CurrencyId::KUSD.symbol(), b"KUSD");
    assert_eq!(CurrencyId::LKSM.symbol(), b"LKSM");
}

#[test]
/// Test try from Vec<u8>.
fn test_try_from() {
    assert_eq!(
        CurrencyId::try_from(b"PONT".to_vec()).unwrap(),
        CurrencyId::PONT
    );
    assert_eq!(
        CurrencyId::try_from(b"KSM".to_vec()).unwrap(),
        CurrencyId::KSM
    );
    assert_eq!(
        CurrencyId::try_from(b"KAR".to_vec()).unwrap(),
        CurrencyId::KAR
    );
    assert_eq!(
        CurrencyId::try_from(b"KUSD".to_vec()).unwrap(),
        CurrencyId::KUSD
    );
    assert_eq!(
        CurrencyId::try_from(b"LKSM".to_vec()).unwrap(),
        CurrencyId::LKSM
    );
    assert_eq!(CurrencyId::try_from(b"UNKNOWN".to_vec()), Err(()));
}
