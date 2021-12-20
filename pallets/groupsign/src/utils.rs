use codec::{Encode};
use sp_io::{hashing::blake2_256};
use frame_support::error::BadOrigin;

/// Ensure this origin represents a groupsign origin
pub fn ensure_groupsign<T, OuterOrigin>(o: OuterOrigin) -> Result<crate::Origin<T>, BadOrigin>
where
    T: crate::Config,
    OuterOrigin: Into<Result<crate::Origin<T>, OuterOrigin>>,
{
    match o.into() {
        Ok(origin) => Ok(origin),
        Err(_) => Err(BadOrigin),
    }
}

pub fn generate_preimage<T: crate::Config>(
    caller: &T::AccountId,
    call: &<T as crate::Config>::Call,
    signers: &[T::AccountId],
    valid_since: T::BlockNumber,
    valid_thru: T::BlockNumber,
) -> [u8; 32] {
    let nonce: <T as frame_system::Config>::Index =
        frame_system::Pallet::<T>::account_nonce(&caller);

    let mut call_preimage = call.encode();
    call_preimage.extend(valid_since.encode());
    call_preimage.extend(valid_thru.encode());
    call_preimage.extend(caller.encode());
    call_preimage.extend(nonce.encode());
    call_preimage.extend(signers.encode());
    blake2_256(call_preimage.as_ref())
}
