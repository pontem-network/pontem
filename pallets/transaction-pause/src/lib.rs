#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::{CallMetadata, GetCallMetadata},
    pallet_prelude::*,
    traits::{Contains, PalletInfoAccess},
    transactional,
};
use frame_system::pallet_prelude::*;
use sp_runtime::DispatchResult;
use sp_std::{prelude::*, vec::Vec};

mod mock;
mod tests;
pub mod weights;

pub use module::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod module {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The origin which may set filter.
        type UpdateOrigin: EnsureOrigin<Self::Origin>;

        /// Weight information for the extrinsics in this module.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);
}
