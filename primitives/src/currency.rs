/// Supported currencies.
use sp_core::RuntimeDebug;
use sp_std::convert::TryFrom;
use sp_std::vec::Vec;
use sp_std::cmp::PartialEq;
use sp_std::default::Default;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::Balance;

#[derive(Default, Debug)]
pub struct CurrencyConversionError(Vec<u8>);

impl CurrencyConversionError {
    fn new(vec: Vec<u8>) -> Self {
        CurrencyConversionError(vec)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CurrencyConversionError {}

impl sp_std::fmt::Display for CurrencyConversionError {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "can't convert {:?} to currency", self.0)
    }
}

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
            pub const fn decimals(&self) -> u8 {
                match self {
                    $(Self::$name => $decimals,)*
                }
            }

            const fn value(&self) -> Balance {
                Balance::pow(10, self.decimals() as _)
            }

            pub const fn times(&self, n: Balance) -> Balance {
                self.value().saturating_mul(n)
            }

            pub const fn millies(self) -> Millies {
                Millies(self)
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
                    _ => Err(Self::Error::new(v)),
                }
            }
        }

        impl TryFrom<&'_ [u8]> for $ty_name {
            type Error = CurrencyConversionError;

            fn try_from(v: &'_ [u8]) -> Result<Self, Self::Error> {
                match v {
                    $($str => Ok(Self::$name),)*
                    _ => Err(Self::Error::new(v.to_vec())),
                }
            }
        }

        /// This PartialEq impl needed to compare enum constructor-pattern with in-enum constant. E.g.:
        /// ```
        /// use primitives::currency::CurrencyId;
        /// let ref currency_id = CurrencyId::KSM;
        /// match currency_id {
        ///     &CurrencyId::NATIVE => {},
        ///     CurrencyId::KSM  => {}
        /// }
        /// ```
        /// Anyway it can be omitted with de-reference (imp-copy) the instance:
        /// ```
        /// use primitives::currency::CurrencyId;
        /// let ref currency_id = CurrencyId::KSM;
        /// match *currency_id {
        ///     CurrencyId::NATIVE => {},
        ///     CurrencyId::KSM  => {}
        /// }
        /// ```
        impl core::cmp::PartialEq<$ty_name> for &'_ $ty_name {
            fn eq(&self, other: &$ty_name) -> bool {
                &other == *self
            }
        }

        $(static_assert!(const_slice_eq(stringify!($name).as_bytes(), $str));)*
    };
}

#[rustfmt::skip]
pub const NATIVE_SYM: &'static [u8] = {
    #[cfg(not(feature = "pont"))] { b"NOX" }
    #[cfg(feature = "pont")] { b"PONT" }
};

#[cfg(not(feature = "pont"))]
def_currencies! {
    /// Currencies id.
    #[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum CurrencyId {
        /// Our native currency.
        NOX(b"NOX", 10),
        /// Relaychain's currency.
        KSM(b"KSM", 12),
    }
}

#[cfg(feature = "pont")]
def_currencies! {
    /// Currencies id.
    #[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum CurrencyId {
        /// Our native currency.
        PONT(b"PONT", 10),
        /// Relaychain's currency.
        KSM(b"KSM", 12),
    }
}

impl Default for CurrencyId {
    fn default() -> Self {
        CurrencyId::native()
    }
}

impl CurrencyId {
    pub const NATIVE: CurrencyId = CurrencyId::NOX;

    // Create a new CurrencyId with native currency.
    pub const fn native() -> Self {
        #[cfg(not(feature = "pont"))]
        {
            CurrencyId::NOX
        }
        #[cfg(feature = "pont")]
        {
            CurrencyId::PONT
        }
    }
}

pub struct Millies(pub CurrencyId);

impl Millies {
    const fn value(&self) -> Balance {
        self.0.value() / 1000
    }

    pub const fn times(&self, n: Balance) -> Balance {
        self.value().saturating_mul(n)
    }
}

impl core::ops::Mul<Balance> for CurrencyId {
    type Output = Balance;

    fn mul(self, n: Balance) -> Self::Output {
        self.times(n)
    }
}

impl core::ops::Mul<Balance> for Millies {
    type Output = Balance;

    fn mul(self, n: Balance) -> Self::Output {
        self.times(n)
    }
}

#[cfg(test)]
mod tests {
    use super::{NATIVE_SYM, CurrencyId, TryFrom};

    #[test]
    /// Test default currency.
    fn default() {
        assert_eq!(CurrencyId::default(), CurrencyId::NATIVE);
    }

    #[test]
    /// Test currencies decimals.
    fn decimals() {
        assert_eq!(CurrencyId::NATIVE.decimals(), 10);
        assert_eq!(CurrencyId::KSM.decimals(), 12);
    }

    #[test]
    /// Test currencies symbols.
    fn symbols() {
        assert_eq!(CurrencyId::KSM.symbol(), b"KSM");
        assert_eq!(CurrencyId::NATIVE.symbol(), CurrencyId::native().symbol());
        assert_eq!(CurrencyId::NATIVE.symbol(), NATIVE_SYM);
    }

    #[test]
    /// Test try from Vec<u8>.
    fn try_from_vec() {
        assert_eq!(
            CurrencyId::try_from(NATIVE_SYM.to_vec()).unwrap(),
            CurrencyId::NATIVE
        );
        assert_eq!(
            CurrencyId::try_from(b"KSM".to_vec()).unwrap(),
            CurrencyId::KSM
        );
        assert!(CurrencyId::try_from(b"UNKNOWN".to_vec()).is_err());
    }

    #[test]
    /// Test try from &[u8].
    fn try_from_slice() {
        assert_eq!(
            CurrencyId::try_from(NATIVE_SYM.as_ref()).unwrap(),
            CurrencyId::NATIVE
        );
        assert_eq!(
            CurrencyId::try_from(b"KSM".as_ref()).unwrap(),
            CurrencyId::KSM
        );
        assert!(CurrencyId::try_from(b"UNKNOWN".as_ref()).is_err());
    }
}
