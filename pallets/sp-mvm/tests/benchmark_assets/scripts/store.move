script {
    use 0x1::Store;

    fun store(sender: signer) {
        Store::store(&sender);
    }
}
