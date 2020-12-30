script {
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_u64(account: &signer, val: u64) {
        Store::store_u64(account, val);
    }
}
