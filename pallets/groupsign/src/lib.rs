#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use frame_support::{dispatch::{self, Dispatchable, GetDispatchInfo}, ensure, pallet_prelude::*};
	use frame_system::{CheckNonce, pallet_prelude::*};
	use sp_runtime::{AccountId32, MultiSignature, MultiSigner, traits::{Lazy, Verify}, verify_encoded_lazy};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: Parameter
             + Dispatchable<Origin = Self::Origin>
             + GetDispatchInfo
             + From<frame_system::Call<Self>>;

	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// SomethingStored(u32, T::AccountId),
		Nada
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// Era validator error (means valid_since, valid_thru don't pass filter).
		EreValidationError,

		// When signatures length doesn't match signers length.
		SignaturesLengthDoesntMatch,

		// Can't verify signature.
		SignatureVerificationError
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		/// Do groupsign call.
		#[pallet::weight(0)]
        pub fn groupsign_call(
            origin: OriginFor<T>,
            signed_call: Box<<T as Config>::Call>,
            signers: Vec<AccountId32>,
			signatures: Vec<MultiSignature>,
			valid_since: T::BlockNumber,
			valid_thru: T::BlockNumber
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			ensure!(
				signatures.len() == signers.len(),
				Error::<T>::SignaturesLengthDoesntMatch
			);

			// Check era.
			let current_block = frame_system::Pallet::<T>::block_number();

			ensure!(
				current_block >= valid_since && current_block < valid_thru,
				Error::<T>::EreValidationError,
			);

			// Get account nonce.
			let nonce = frame_system::Pallet::<T>::account_nonce(&caller);

			// Generate signature.
			let mut call_preimage = signed_call.encode();
			call_preimage.extend(valid_since.encode());
			call_preimage.extend(valid_thru.encode());
			call_preimage.extend(caller.encode());
			call_preimage.extend(nonce.encode());

			// Verify signature.
			let verified = Iterator::zip(signatures.into_iter(), signers.into_iter())
				.all(|(sig, signer)| verify_encoded_lazy(&sig, &call_preimage, &signer));

			ensure!(verified, Error::<T>::SignatureVerificationError);
			
			// Do dispatch call.
			Ok(())
        }

	}

}
