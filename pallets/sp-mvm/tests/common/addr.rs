#![allow(dead_code)]
/// Mock addresses.
use parity_scale_codec::Encode;
use sp_core::sr25519::Public;
use sp_core::crypto::Ss58Codec;
use move_core_types::account_address::AccountAddress;
pub use move_core_types::language_storage::CORE_CODE_ADDRESS as ROOT_ADDR;
use sp_mvm::addr::account_to_account_address;

pub const BOB_SS58: &str = "gkNW9pAcCHxZrnoVkhLkEQtsLsW5NWTC75cdAdxAMs9LNYCYg";
pub const ALICE_SS58: &str = "gkQ5K6EnLRgZkwozG8GiBAEnJyM6FxzbSaSmVhKJ2w8FcK7ih";

/// Public key of Bob account.
pub fn bob_public_key() -> Public {
    Public::from_ss58check_with_version(BOB_SS58).unwrap().0
}

/// Public key of alice account.
pub fn alice_public_key() -> Public {
    Public::from_ss58check_with_version(ALICE_SS58).unwrap().0
}

/// Returns `AccountAddress` for Bob.
pub fn origin_move_addr() -> AccountAddress {
    let pk = bob_public_key();
    let vec = pk.encode();
    let mut arr = [0; AccountAddress::LENGTH];
    arr.copy_from_slice(&vec);
    AccountAddress::new(arr)
}

/// Returns `AccountAddress` for Alice.
pub fn alice_move_addr() -> AccountAddress {
    let pk = alice_public_key();
    let vec = pk.encode();
    let mut arr = [0; AccountAddress::LENGTH];
    arr.copy_from_slice(&vec);
    AccountAddress::new(arr)
}

/// Returns `AccountAddress` for provided public key.
pub fn to_move_addr(pk: Public) -> AccountAddress {
    account_to_account_address(&pk)
}

#[cfg(test)]
mod tests {
    #[test]
    fn origin_move_addr() {
        let expected = super::AccountAddress::from_hex_literal(
            "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
        )
        .unwrap();
        assert_eq!(expected, super::origin_move_addr());
    }

    #[test]
    fn origin_to_move_addr() {
        use super::*;

        let expected = AccountAddress::from_hex_literal(
            "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
        )
        .unwrap();
        let addr = to_move_addr(bob_public_key());
        assert_eq!(expected, addr);
    }
}
