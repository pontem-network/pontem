/// Supported currencies.
/// TODO: would be good to replace with Acala currencies: https://github.com/AcalaNetwork/Acala/blob/master/primitives/src/currency.rs.
/// Or implement something similar (without EVM/DEX, etc.)
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;
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
}

/// Implement currencies.
impl CurrencyId {
    pub fn decimals(&self) -> u8 {
        match self {
            Self::KSM => 12,
            Self::PONT => 10,
        }
    }

    pub fn symbol(&self) -> Vec<u8> {
        match self {
            Self::KSM => b"KSM".to_vec(),
            Self::PONT => b"PONT".to_vec(),
        }
    }
}
