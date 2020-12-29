script {
    use 0x1::Store;

    fun store_u64(account: &signer, val: u64) {
        Store::store_u64(account, val);
    }
}
