script {
    use 0x1::Store;

    fun store(signer: &signer) {
        Store::store(signer);
    }
}