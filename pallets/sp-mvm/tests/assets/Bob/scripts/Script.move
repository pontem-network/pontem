script {
    use 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48::Store;

    fun store_u64(account: &signer, val: u64) {
        Store::store_u64(account, val);
    }
}
