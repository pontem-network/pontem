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

impl<T> AccountIdAsBytes<T::AccountId, [u8; AccountAddress::LENGTH]> for T
where
    T: frame_system::Trait,
    T::AccountId: Encode,
{
    fn account_to_bytes(acc: &T::AccountId) -> [u8; AccountAddress::LENGTH] {
        trace!("converting account: {:?}", acc);
        const LENGTH: usize = AccountAddress::LENGTH;
        let mut result = [0; LENGTH];
        let bytes = acc.encode();

        trace!("  account (pk) bytes: {:?}", bytes);

        let skip = if bytes.len() < LENGTH {
            LENGTH - bytes.len()
        } else {
            0
        };

        (&mut result[skip..]).copy_from_slice(&bytes);

        trace!("  result bytes: (skip bytes: {}) {:?}", skip, result);
        result
    }
}
