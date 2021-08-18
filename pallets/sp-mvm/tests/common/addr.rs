#![allow(dead_code)]

use parity_scale_codec::Encode;
use sp_core::sr25519::Public;
use sp_core::crypto::Ss58Codec;
use move_core_types::account_address::AccountAddress;
pub use move_core_types::language_storage::CORE_CODE_ADDRESS as ROOT_ADDR;
use sp_mvm::addr::account_to_account_address;

pub const BOB_SS58: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
pub const ALICE_SS58: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

/// Returns pk for //Bob (5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty)
pub fn origin_ps_acc() -> Public {
    let pk = Public::from_ss58check(BOB_SS58).unwrap();
    pk
}

pub fn bob_public_key() -> Public {
    Public::from_ss58check(BOB_SS58).unwrap()
}

pub fn alice_public_key() -> Public {
    Public::from_ss58check(ALICE_SS58).unwrap()
}

/// Returns `AccountAddress` for Bob
pub fn origin_move_addr() -> AccountAddress {
    let pk = origin_ps_acc();
    let vec = pk.encode();
    let mut arr = [0; AccountAddress::LENGTH];
    arr.copy_from_slice(&vec);
    AccountAddress::new(arr)
}

pub fn alice_move_addr() -> AccountAddress {
    let pk = alice_public_key();
    let vec = pk.encode();
    let mut arr = [0; AccountAddress::LENGTH];
    arr.copy_from_slice(&vec);
    AccountAddress::new(arr)
}

pub fn root_ps_acc() -> Public {
    let addr = ROOT_ADDR;
    let pk = Public(addr.to_u8());
    pk
}

/// Returns `AccountAddress` for Bob
pub fn root_move_addr() -> AccountAddress {
    let pk = root_ps_acc();
    let vec = pk.encode();
    let mut arr = [0; AccountAddress::LENGTH];
    arr.copy_from_slice(&vec);
    AccountAddress::new(arr)
}

/// Returns `AccountAddress` for Bob
pub fn to_move_addr(pk: Public) -> AccountAddress {
    account_to_account_address(&pk)
}

#[cfg(test)]
mod tests {
    #[test]
    fn root_move_addr() {
        use move_core_types::language_storage::CORE_CODE_ADDRESS;
        assert_eq!(CORE_CODE_ADDRESS, super::root_move_addr());
    }

    #[test]
    fn root_ps_acc() {
        use move_core_types::language_storage::CORE_CODE_ADDRESS;
        assert_eq!(CORE_CODE_ADDRESS.to_u8(), super::root_ps_acc().0);
    }

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
        let addr = to_move_addr(origin_ps_acc());
        assert_eq!(expected, addr);
    }
}
