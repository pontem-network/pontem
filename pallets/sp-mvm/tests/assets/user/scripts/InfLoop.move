script {
    fun inf_loop() {
        let i = 0;

        loop {
            if (i > 1000) {
                break
            }
        }
    }
}
