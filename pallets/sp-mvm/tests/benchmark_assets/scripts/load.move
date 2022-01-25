script {
    use Benchmarks::Store;

    fun load() {
        if (Store::exisit()) {
            Store::borrow();
        } else {
            abort 1
        }
    }
}
