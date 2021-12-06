use codec::{Encode};
use sp_io::{hashing::blake2_256};
use sp_std::prelude::*;

#[cfg(feature = "benchmarking")]
use frame_benchmarking;
#[cfg(feature = "benchmarking")]
use sp_core::{Pair, sr25519};
#[cfg(feature = "benchmarking")]
use sp_std::vec::Vec;

// benchmark/test methods
// each method works on reproducible keys
#[cfg(feature = "benchmarking")]
pub(crate) fn reencode<A: Encode, B: Decode>(a: A, buf: &mut [u8]) -> Result<B, codec::Error> {
    a.using_encoded(|f| buf.copy_from_slice(f));
    B::decode(&mut &buf[..])
}
#[cfg(feature = "benchmarking")]
pub(crate) fn test_pairs(range: Range<u32>) -> impl Iterator<Item = sr25519::Pair> {
    range.map(|acc| sr25519::Pair::from_entropy(&acc.to_be_bytes(), None).0)
}
#[cfg(feature = "benchmarking")]
pub(crate) fn test_accounts<T: crate::Config>(range: Range<u32>) -> Vec<T::AccountId> {
    test_pairs(range).map(|s| {
        reencode(s.public(), &mut [0u8; 32]).expect("Decoded account")
        // let mut buf = [0u8; 32];
        // s.public().using_encoded(|f| buf.copy_from_slice(f));
        // T::AccountId::decode(&mut &buf[..]).expect("Decoded account")
    }).collect()
}
#[cfg(feature = "benchmarking")]
pub(crate) fn test_sign<T: crate::Config>(range: Range<u32>, message: &[u8]) -> Vec<T::Signature>  {
    test_pairs(range).map(|s| {
        reencode(s.sign(message), &mut [0u8; 32]).expect("Decoded signature")
        // let mut buf = [0u8; 32];
        // s.sign(message).using_encoded(|f| buf.copy_from_slice(f));
        // T::Signature::decode(&mut &buf[..]).expect("Decoded signature")
    }).collect()
}

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
