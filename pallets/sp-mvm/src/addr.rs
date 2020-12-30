use sp_std::prelude::*;
use codec::Encode;
use move_core_types::account_address::AccountAddress;

pub trait AccountIdAsBytes<AccountId, T: Sized> {
    fn account_to_bytes(acc: &AccountId) -> T;
}

impl<T> AccountIdAsBytes<T::AccountId, Vec<u8>> for T
where
    T: frame_system::Trait,
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

pub fn account_to_account_address<AccountId: Encode>(acc: &AccountId) -> AccountAddress {
    AccountAddress::new(account_to_bytes(acc))
}

impl<T> AccountIdAsBytes<T::AccountId, [u8; AccountAddress::LENGTH]> for T
where
    T: frame_system::Trait,
    T::AccountId: Encode,
{
    fn account_to_bytes(acc: &T::AccountId) -> [u8; AccountAddress::LENGTH] {
        account_to_bytes(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::account_to_account_address;
    use sp_core::sr25519::Public;
    use sp_core::crypto::Ss58Codec;

    #[test]
    fn convert_address() {
        //Alice
        let pk =
            Public::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();
        let addr = account_to_account_address(&pk);
        assert_eq!(
            "D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D",
            addr.to_string()
        );

        //Bob
        let pk =
            Public::from_ss58check("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap();
        let addr = account_to_account_address(&pk);
        assert_eq!(
            "8EAF04151687736326C9FEA17E25FC5287613693C912909CB226AA4794F26A48",
            addr.to_string()
        );
    }
}
