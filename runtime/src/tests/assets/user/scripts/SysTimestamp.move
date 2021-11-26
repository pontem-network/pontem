script {
    use 0x1::DiemTimestamp;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_system_timestamp(account: signer) {
        Store::store_u64(&account, DiemTimestamp::now_microseconds());
    }
}
