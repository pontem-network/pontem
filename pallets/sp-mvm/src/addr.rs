use sp_std::prelude::*;
use sp_core::U256;
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
        debug!("converting account: {:?}", acc);
        const LENGTH: usize = AccountAddress::LENGTH;
        let mut result = [0_u8; LENGTH];
        let bytes = acc.encode();

        debug!("  acc bytes: {:?}", bytes);

        let skip = if bytes.len() < LENGTH {
            LENGTH - bytes.len()
        } else {
            0
        };

        debug!("  skip bytes: {:?}", skip);

        let u256 = U256::from_little_endian(&bytes[..]);
        debug!("  u256: {:?}", u256);
        u256.to_big_endian(&mut result[skip..]);

        debug!("  result bytes: {:?}", result);
        result
    }
}
