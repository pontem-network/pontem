script {
    use 0x1::DiemBlock;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_system_block(account: signer) {
        Store::store_u64(&account, DiemBlock::get_current_block_height());
    }
}
