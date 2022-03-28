script {
    use PontemFramework::PontTimestamp;
    use UserTests::Store;

    fun store_system_timestamp(account: signer) {
        Store::store_u64(&account, PontTimestamp::now_microseconds());
    }
}
