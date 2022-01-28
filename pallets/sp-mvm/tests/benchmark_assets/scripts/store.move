script {
    use Benchmarks::Store;

    fun store(sender: signer) {
        Store::store(&sender);
    }
}
