use sp_std::prelude::*;
use frame_system as system;
use codec::{Encode, Decode, Error};
use move_core_types::account_address::AccountAddress;

pub trait AccountIdAsBytes<AccountId, T: Sized> {
    fn account_to_bytes(acc: &AccountId) -> T;
}

impl<T> AccountIdAsBytes<T::AccountId, Vec<u8>> for T
where
    T: system::Config,
    T::AccountId: Encode,
{
    fn account_to_bytes(acc: &T::AccountId) -> Vec<u8> {
        acc.encode()
    }
}

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

pub fn address_to_account<AccountId>(address: &AccountAddress) -> Result<AccountId, Error>
where
    AccountId: Decode + Sized,
{
    AccountId::decode(&mut address.as_ref())
}

pub fn account_to_account_address<AccountId: Encode>(acc: &AccountId) -> AccountAddress {
    AccountAddress::new(account_to_bytes(acc))
}

impl<T> AccountIdAsBytes<T::AccountId, [u8; AccountAddress::LENGTH]> for T
where
    T: system::Config,
    T::AccountId: Encode,
{
    #[inline]
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
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D",
    );
    const BOB: (&str, &str) = (
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "8EAF04151687736326C9FEA17E25FC5287613693C912909CB226AA4794F26A48",
    );
    const STD: (&str, &str) = (
        "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUqAsg",
        "0000000000000000000000000000000000000000000000000000000000000001",
    );
    const ALL: &'static [(&str, &str)] = &[ALICE, BOB, STD];

    #[test]
    fn convert_address() {
        for pair in ALL.iter() {
            let pk = Public::from_ss58check(pair.0).unwrap();
            let addr = account_to_account_address(&pk);
            assert_eq!(pair.1, addr.to_string());
        }
    }

    #[test]
    fn convert_address_revert() {
        for pair in ALL.iter() {
            let pk_expected = Public::from_ss58check(pair.0).unwrap();
            let addr = account_to_account_address(&pk_expected);
            let pk_decoded = address_to_account(&addr).expect("Cannot decode address");
            assert_eq!(pk_expected, pk_decoded);
        }
    }

    #[test]
    fn account_to_bytes() {
        for pair in ALL.iter() {
            let pk_expected = Public::from_ss58check(pair.0).unwrap();
            let bytes = super::account_to_bytes(&pk_expected);
            let bytes_expected = AccountAddress::from_hex_literal(&format!("0x{}", pair.1))
                .expect("Cannot decode address, this cannot be, so unreachable.")
                .to_u8();
            assert_eq!(bytes_expected, bytes);
        }
    }
}
