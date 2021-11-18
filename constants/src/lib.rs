#![cfg_attr(not(feature = "std"), no_std)]
/// Constants for Pontem runtime.

/// SS58 PREFIX.
pub const SS58_PREFIX: u8 = 105;

/// Module contains time constants.
pub mod time;

/// Module contains currency constants.
pub mod currency;
