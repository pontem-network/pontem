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

pub type CurrencyConversionError = ();

#[allow(dead_code)]
const fn const_slice_eq(a: &[u8], b: &[u8]) -> bool {
    let len = a.len();
    if b.len() != len {
        return false;
    }
    let mut i = 0;
    while i < len {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

macro_rules! static_assert {
    ($cond:expr) => {
        #[deny(const_err)]
        const _: [(); 1] = [(); $cond as usize];
    };
}

macro_rules! def_currencies {
    (
        $(#[$ty_attr:meta])*
        $vis:vis enum $ty_name:ident {
            $(
                $(#[$attr:meta])*
                $name:ident($str:literal, $decimals:expr)
            ),*
            $(,)?
        }
    ) => {
        $(#[$ty_attr])*
        $vis enum $ty_name {
            $(
                $(#[$attr])*
                $name,
            )*
        }

        impl $ty_name {
            pub fn decimals(&self) -> u8 {
                match self {
                    $(Self::$name => $decimals,)*
                }
            }

            pub fn symbol(&self) -> Vec<u8> {
                match self {
                    $(Self::$name => $str.to_vec(),)*
                }
            }
        }

        impl TryFrom<Vec<u8>> for $ty_name {
            type Error = CurrencyConversionError;

            fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
                match &v[..] {
                    $($str => Ok(Self::$name),)*
                    _ => Err(()),
                }
            }
        }

        $(static_assert!(const_slice_eq(stringify!($name).as_bytes(), $str));)*
    };
}

def_currencies! {
    /// Currencies id.
    #[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum CurrencyId {
        /// Relaychain's currency.
        KSM(b"KSM", 12),
        /// Our native currency.
        PONT(b"PONT", 10),
    }
}

impl Default for CurrencyId {
    fn default() -> Self {
        CurrencyId::PONT
    }
}
