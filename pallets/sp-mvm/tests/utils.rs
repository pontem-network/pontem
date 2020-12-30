#![allow(dead_code)]

use codec::Encode;
use sp_core::sr25519::Public;
use sp_core::crypto::Ss58Codec;
use move_core_types::account_address::AccountAddress;
use sp_mvm::addr::account_to_account_address;

pub const BOB_SS58: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

/// Returns pk for //Bob (5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty)
pub fn origin_ps_acc() -> Public {
    let pk = Public::from_ss58check(BOB_SS58).unwrap();
    pk
}

/// Returns `AccountAddress` for Bob
pub fn origin_move_addr() -> AccountAddress {
    let pk = origin_ps_acc();
    let vec = pk.encode();
    let mut arr = [0; AccountAddress::LENGTH];
    arr.copy_from_slice(&vec);
    AccountAddress::new(arr)
}

pub fn root_ps_acc() -> Public {
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();
    let pk = Public(addr.to_u8());
    pk
}

/// Returns `AccountAddress` for Bob
pub fn root_move_addr() -> AccountAddress {
    let pk = origin_ps_acc();
    let vec = pk.encode();
    let mut arr = [0; AccountAddress::LENGTH];
    arr.copy_from_slice(&vec);
    AccountAddress::new(arr)
}

/// Returns `AccountAddress` for Bob
pub fn to_move_addr(pk: Public) -> AccountAddress {
    account_to_account_address(&pk)
}
