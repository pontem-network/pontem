module Store {
    resource struct U64 {val: u64}
    resource struct U128 {val: u128}
    resource struct Address {val: address}
    resource struct VectorU8 {val: vector<u8>}

    public fun store_u64(account: &signer, val: u64) {
        let foo = U64 {val: val};
        move_to<U64>(account, foo);
    }

    public fun store_u128(account: &signer, val: u128) {
        let foo = U128 {val: val};
        move_to<U128>(account, foo);
    }

    public fun store_address(account: &signer, val: address) {
        let addr = Address {val: val};
        move_to<Address>(account, addr);
    }

    public fun store_vector_u8(account: &signer, val: vector<u8>) {
        let vec = VectorU8 {val: val};
        move_to<VectorU8>(account, vec);
    }
}