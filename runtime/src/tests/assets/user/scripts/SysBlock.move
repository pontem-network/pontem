script {
    use PontemFramework::PontBlock;
    use RuntimeTests::Store;

    fun store_system_block(account: signer) {
        Store::store_u64(&account, PontBlock::get_current_block_height());
    }
}

