#![cfg_attr(not(feature = "std"), no_std)]
/// Primitives types for Pontem runtime.
/// Contains runtime primitvies and types to resolve currrencies.
use sp_runtime::{generic, MultiSignature, OpaqueExtrinsic as UncheckedExtrinsic};
use sp_runtime::traits::{Verify, IdentifyAccount, BlakeTwo256};

pub mod currency;

/// Block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// Block header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u64;

// Signed version of `Balance` for xtokens.
pub type Amount = i64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem;
