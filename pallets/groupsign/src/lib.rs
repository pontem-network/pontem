#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
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
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	// #[pallet::storage]
	// #[pallet::getter(fn something)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// SomethingStored(u32, T::AccountId),
		Nada
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		// NoneValue,
		/// Errors should have helpful documentation associated with them.
		// StorageOverflow,
		SignatureVerificationError
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// ///
		// #[pallet::weight(0)]
		// pub fn do_groupsign(
		// 	origin: OriginFor<T>, test: u64
		// ) -> DispatchResult {
		// 	Ok(())
		// }

		///
		#[pallet::weight(0)]
        pub fn groupsign_call(
            origin: OriginFor<T>,
            signed_call: Box<<T as Config>::Call>,
            signers: Vec<AccountId32>,
			signatures: Vec<MultiSignature>,
			valid_since: T::BlockNumber,
			valid_thru: T::BlockNumber
		) -> DispatchResult {
			// TODO: Validate era

			if signatures.len() != signers.len() {
				return Err(DispatchError::Other("Signatures number does not correspond to signers number"))
			}

			let caller = ensure_signed(origin)?;

			let mut call_preimage = signed_call.encode();
			call_preimage.extend(valid_since.encode());
			call_preimage.extend(valid_thru.encode());
			call_preimage.extend(caller.encode());

			// Get account nonce
			let _ = frame_system::Pallet::<T>::account_nonce(caller);

			// STOPSHIP: Add caller nonce, this is important!
			// call_preimage.extend(frame_system::Account::<T>::get(&caller).nonce.encode());

			let verified = Iterator::zip(signatures.into_iter(), signers.into_iter())
				.all(|(sig, signer)| verify_encoded_lazy(&sig, &call_preimage, &signer));

			if verified {
				// Dispatch a call with all the verified accounts.
				Ok(())
			} else {
				return Err(DispatchError::Other("Signature "))
			}

        }

	}

}
