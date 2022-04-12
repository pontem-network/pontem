module RuntimeTests::Store {
    struct U64 has key { val: u64 }

    struct U128 has key { val: u128 }

    struct Address has key { val: address }

    struct VectorU8 has key { val: vector<u8> }

    public fun store_u64(account: &signer, val: u64) {
        let foo = U64 { val: val };
        move_to<U64>(account, foo);
    }

    public fun store_u128(account: &signer, val: u128) {
        let foo = U128 { val: val };
        move_to<U128>(account, foo);
    }

    public fun store_address(account: &signer, val: address) {
        let addr = Address { val: val };
        move_to<Address>(account, addr);
    }

    public fun store_vector_u8(account: &signer, val: vector<u8>) {
        let vec = VectorU8 { val: val };
        move_to<VectorU8>(account, vec);
    }
}
