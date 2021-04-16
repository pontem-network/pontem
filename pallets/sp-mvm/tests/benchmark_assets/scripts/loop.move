script {
    fun lp() {
        let couter = 500000;
        loop {
            couter = couter - 1;
            if (couter < 1) {
                break
            };
        }
    }
}