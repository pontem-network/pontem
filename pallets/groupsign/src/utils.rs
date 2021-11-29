use codec::Encode;
use sp_core::blake2_256;


pub fn generate_preimage<T: crate::Config>(
    caller: &T::AccountId,
    call: &<T as crate::Config>::Call,
    signers: &Vec<T::AccountId>,
    valid_since: T::BlockNumber,
    valid_thru: T::BlockNumber
) -> [u8; 32] {
    let nonce = frame_system::Pallet::<T>::account_nonce(&caller);

    let mut call_preimage = call.encode();
    call_preimage.extend(valid_since.encode());
    call_preimage.extend(valid_thru.encode());
    call_preimage.extend(caller.encode());
    call_preimage.extend(nonce.encode());

    // We collect check that signers didn't changed.
    call_preimage.extend(signers.encode());
    blake2_256(call_preimage.as_ref())
}
