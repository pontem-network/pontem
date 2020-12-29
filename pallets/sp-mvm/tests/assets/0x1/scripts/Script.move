script {
    use 0x00000000000000000000000000000000000000000000000000000000000000000001::Store;

    fun store_u64(account: &signer, val: u64) {
        Store::store_u64(account, val);
    }
}
