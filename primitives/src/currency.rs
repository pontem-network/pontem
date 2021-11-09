use sp_core::RuntimeDebug;
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
    PONT
}

impl CurrencyId {
    pub fn decimals(&self) -> Option<u8> {
        match self {
            Self::KSM => Some(12),
            Self::PONT => Some(10)
        }
    }
}
