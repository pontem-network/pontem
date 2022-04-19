// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! This pallet enables executing dispatchable calls by using several signers and their signatures.
//! Executed calls have the option to get signers inside by using `T::Origin` as origin from the current pallet.
//! It's useful for some kinds of multisignatures implementations, e.g. Move VM supports multisignature out of the box,
//! yet it asks for signers of the current transaction.
//! Signers should sign hash `(blake2_256)` generated from data contains encoded: `call`, `valid_since`, `valid_thru`, `caller`, `nonce`.
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod utils;
pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use sp_std::prelude::Box;
    // use scale_info::TypeInfo;
    use frame_support::scale_info::TypeInfo;
    use frame_support::{
        dispatch::{DispatchResultWithPostInfo, Dispatchable, GetDispatchInfo, PostDispatchInfo},
        ensure,
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    use sp_runtime::{
        traits::{Verify, IdentifyAccount},
        verify_encoded_lazy,
    };

    use crate::weights::WeightInfo;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Events.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Origin.
        type MyOrigin: From<Origin<Self>> + Into<<Self as frame_system::Config>::Origin>;

        /// Call types.
        type Call: Parameter
            + Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>;

        /// Public key type.
        /// Inspiration - https://github.com/JoshOrndorff/recipes/issues/142
        type Public: IdentifyAccount<AccountId = Self::AccountId>
            + Clone
            + TypeInfo
            + Encode
            + Decode
            + PartialEq
            + sp_std::fmt::Debug;

        /// Signature type.
        /// Inspiration - https://github.com/JoshOrndorff/recipes/issues/142
        type Signature: Verify<Signer = Self::Public>
            + Member
            + Decode
            + Encode
            + TypeInfo
            + PartialEq
            + sp_std::fmt::Debug;

        type WeightInfo: WeightInfo;
    }

    #[pallet::origin]
    #[derive(PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct Origin<T: Config> {
        pub caller: T::AccountId,
        pub signers: Vec<T::AccountId>,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// When dispatchable executed.
        DispatchableExecuted(
            // Caller.
            T::AccountId,
            // Hash of call data.
            Vec<u8>,
        ),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        // Era validator error (means valid_since, valid_thru don't pass filter).
        EraValidationError,

        // When signatures length doesn't match signers length.
        SignaturesLengthDoesntMatch,

        // When zero signatures provided.
        ZeroSignatureCall,

        // Can't verify signature.
        SignatureVerificationError,

        // Can't execute call.
        ExecutionFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Do groupsign call.

        #[pallet::weight({
            let dispatch_info = signed_call.get_dispatch_info();
            (

                T::WeightInfo::groupsign_call(
                    signers.len() as u32,
                    signed_call.using_encoded(|c| c.len() as u32)
                ).saturating_add(dispatch_info.weight),

                dispatch_info.class,

            )
        })]
        pub fn groupsign_call(
            origin: OriginFor<T>,
            signed_call: Box<<T as Config>::Call>,
            signers: Vec<T::AccountId>,
            signatures: Vec<T::Signature>,
            valid_since: T::BlockNumber,
            valid_thru: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;

            // Check signatures length match.
            ensure!(!signatures.is_empty(), Error::<T>::ZeroSignatureCall);

            // Check signatures length match.
            ensure!(
                signatures.len() == signers.len(),
                Error::<T>::SignaturesLengthDoesntMatch
            );

            // Check era.
            let current_block = frame_system::Pallet::<T>::block_number();

            ensure!(
                current_block >= valid_since && current_block < valid_thru,
                Error::<T>::EraValidationError,
            );

            let preimage = crate::utils::generate_preimage::<T>(
                &caller,
                &signed_call,
                &signers,
                valid_since,
                valid_thru,
            );

            // Verify signature.
            let verified = Iterator::zip(signatures.into_iter(), signers.clone().into_iter())
                .all(|(sig, signer)| verify_encoded_lazy(&sig, &preimage, &signer));

            ensure!(verified, Error::<T>::SignatureVerificationError);

            // Needed for weight function
            let call_len = signed_call.using_encoded(|c| c.len());

            // Do dispatch call.
            let origin = Origin {
                caller: caller.clone(),
                signers: signers.clone(),
            };
            let result = signed_call.dispatch(T::MyOrigin::from(origin).into()); // result

            let call_weight = match result {
                Ok(post_info) => {
                    <Pallet<T>>::deposit_event(Event::DispatchableExecuted(
                        caller,
                        preimage.to_vec(),
                    ));
                    post_info.actual_weight
                }
                Err(err) => err.post_info.actual_weight,
            };

            Ok(call_weight
                .map(|actual_weight| {
                    T::WeightInfo::groupsign_call(signers.len() as u32, call_len as u32)
                        .saturating_add(actual_weight)
                })
                .into())
        }
    }
}
