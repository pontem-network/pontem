// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! By default Move VM supports only 0x{hex} address format, which has 32 bytes length.
//! To be in compatibility with Substrate SS58 addresses we implemented traits and functions that allow us to convert SS58 addresses to 0x{hex} format.
//! Current file contains traits and functions which allows us to convert SS58 addresses to 0x{hex} format.

use sp_std::prelude::*;
use frame_system as system;
use parity_scale_codec::{Encode, Decode, Error};
use move_core_types::account_address::AccountAddress;

/// Trait that allows to represent AccountId as bytes.
pub trait AccountIdAsBytes<AccountId, T: Sized> {
    fn account_to_bytes(acc: &AccountId) -> T;
}

/// Implementation of AccountIdAsBytes for AccountId and address as vector.
impl<T> AccountIdAsBytes<T::AccountId, Vec<u8>> for T
where
    T: system::Config,
    T::AccountId: Encode,
{
    fn account_to_bytes(acc: &T::AccountId) -> Vec<u8> {
        acc.encode()
    }
}

/// Convert AccountId to Move VM address format.
///
/// Returns a slice that could be represented as a Move VM address.
/// If provided AccountId length is less than expected address length, fills address with zeros.
/// Otherwise just copy bytes we need.
pub fn account_to_bytes<AccountId: Encode>(acc: &AccountId) -> [u8; AccountAddress::LENGTH] {
    const LENGTH: usize = AccountAddress::LENGTH;
    let mut result = [0; LENGTH];
    let bytes = acc.encode();

    let skip = if bytes.len() < LENGTH {
        LENGTH - bytes.len()
    } else {
        0
    };

    (&mut result[skip..]).copy_from_slice(&bytes);

    trace!(
        "converted: (with skip: {})\n\t{:?}\n\tto {:?}",
        skip,
        bytes,
        result
    );

    result
}

/// Convert Move VM address back to AccountId.
pub fn address_to_account<AccountId>(address: &AccountAddress) -> Result<AccountId, Error>
where
    AccountId: Decode + Sized,
{
    AccountId::decode(&mut address.as_ref())
}

// Create Move VM address instance (AccountAddress) from an AccountId.
pub fn account_to_account_address<AccountId: Encode>(acc: &AccountId) -> AccountAddress {
    AccountAddress::new(account_to_bytes(acc))
}

impl<T> AccountIdAsBytes<T::AccountId, [u8; AccountAddress::LENGTH]> for T
where
    T: system::Config,
    T::AccountId: Encode,
{
    #[inline]
    /// Implementation of AccountIdAsBytes for AccountId and address as slice.
    fn account_to_bytes(acc: &T::AccountId) -> [u8; AccountAddress::LENGTH] {
        account_to_bytes(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::address_to_account;
    use super::account_to_account_address;
    use super::AccountAddress;
    use sp_core::sr25519::Public;
    use sp_core::crypto::Ss58Codec;

    // Expected data for tests
    // pair: (SS58, public key / AccountId)
    const ALICE: (&str, &str) = (
        "gkQ5K6EnLRgZkwozG8GiBAEnJyM6FxzbSaSmVhKJ2w8FcK7ih",
        "D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D",
    );
    const BOB: (&str, &str) = (
        "gkNW9pAcCHxZrnoVkhLkEQtsLsW5NWTC75cdAdxAMs9LNYCYg",
        "8EAF04151687736326C9FEA17E25FC5287613693C912909CB226AA4794F26A48",
    );
    const STD: (&str, &str) = (
        "gkKH52LJ2UumhVBim1n3mCsSj3ctj3GkV8JLVLdhJakxmEDcq",
        "0000000000000000000000000000000000000000000000000000000000000001",
    );
    const ALL: &'static [(&str, &str)] = &[ALICE, BOB, STD];

    #[test]
    fn convert_address() {
        for pair in ALL.iter() {
            let pk = Public::from_ss58check_with_version(pair.0).unwrap().0;
            let addr = account_to_account_address(&pk);
            assert_eq!(pair.1, addr.to_string());
        }
    }

    #[test]
    fn convert_address_revert() {
        for pair in ALL.iter() {
            let pk_expected = Public::from_ss58check_with_version(pair.0).unwrap().0;
            let addr = account_to_account_address(&pk_expected);
            let pk_decoded = address_to_account(&addr).expect("Cannot decode address");
            assert_eq!(pk_expected, pk_decoded);
        }
    }

    #[test]
    fn account_to_bytes() {
        for pair in ALL.iter() {
            let pk_expected = Public::from_ss58check_with_version(pair.0).unwrap().0;
            let bytes = super::account_to_bytes(&pk_expected);
            let bytes_expected = AccountAddress::from_hex_literal(&format!("0x{}", pair.1))
                .expect("Cannot decode address, this cannot be, so unreachable.")
                .to_u8();
            assert_eq!(bytes_expected, bytes);
        }
    }
}
