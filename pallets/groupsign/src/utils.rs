use codec::{Encode};
use sp_io::{hashing::blake2_256};
use sp_std::prelude::*;
use frame_support::{error::BadOrigin, dispatch::Weight};
use sp_runtime::{traits::Saturating, MultiSigner};

use crate::weights::WeightInfo;

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

pub enum CryptoType {
    Sr25519 = 0,
    Ed25519 = 1,
    EcDSA = 2
}

pub trait IdentifyCryptoAlgorithm<T: crate::Config> {
    fn get_crypto_algo(identify: T::AccountId) -> CryptoType;
}

pub struct IdentSr25519();
pub struct IdentEd25519();
pub struct IdentEcdsa();

impl <T> IdentifyCryptoAlgorithm<T> for IdentSr25519 where T: crate::Config  {fn get_crypto_algo(identify: T::AccountId) -> CryptoType  {CryptoType::Sr25519}}
impl <T> IdentifyCryptoAlgorithm<T> for IdentEd25519 where T: crate::Config  {fn get_crypto_algo(identify: T::AccountId) -> CryptoType  {CryptoType::Ed25519}}
impl <T> IdentifyCryptoAlgorithm<T> for IdentEcdsa where T: crate::Config {fn get_crypto_algo(identify: T::AccountId) -> CryptoType  {CryptoType::EcDSA}}

pub struct MultiSignerIdentifier();
impl <T> IdentifyCryptoAlgorithm<T> for MultiSignerIdentifier where T: crate::Config<AccountId = MultiSigner> {
    fn get_crypto_algo(identify: MultiSigner) -> CryptoType {
        match identify {
            MultiSigner::Sr25519(_) => CryptoType::Sr25519,
            MultiSigner::Ed25519(_) => CryptoType::Ed25519,
            MultiSigner::Ecdsa(_) => CryptoType::EcDSA,
        }
    }
}

pub fn calculate_weights<T: crate::Config>(signers: &Vec<T::AccountId>, length: u32) -> Weight {
    let (sr, ed, ec) = signers.iter().fold(
        (0,0,0),
        |(sr, ed, ec), account| {
            match T::IdentifyCryptoAlgorithm::get_crypto_algo(account.clone()) {
                CryptoType::Sr25519 => (sr + 1, ed, ec),
                CryptoType::Ed25519 => (sr, ed + 1, ec),
                CryptoType::EcDSA => (sr, ed, ec + 1),
            }
        });

    T::WeightInfo::on_chain_message_check_sr25519(sr, length)
}
