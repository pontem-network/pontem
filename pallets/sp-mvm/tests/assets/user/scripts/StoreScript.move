script {
    use UserTests::Store;

    fun store_u64(account: signer, val: u64) {
        Store::store_u64(&account, val);
    }
}
