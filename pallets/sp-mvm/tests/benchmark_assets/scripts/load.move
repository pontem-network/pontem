script {
    use 0x1::Store;

    fun load() {
        if (Store::exisit()) {
            Store::borrow();
        } else {
            abort 1
        }
    }
}