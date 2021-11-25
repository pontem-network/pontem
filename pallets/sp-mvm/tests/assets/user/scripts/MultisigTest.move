script {
    use 0x1::Signer;

    fun multisig_test(account1: signer, account2: signer) {
        assert(Signer::address_of(&account1) == @gkNW9pAcCHxZrnoVkhLkEQtsLsW5NWTC75cdAdxAMs9LNYCYg, 1);
        assert(Signer::address_of(&account2) == @gkQ5K6EnLRgZkwozG8GiBAEnJyM6FxzbSaSmVhKJ2w8FcK7ih, 1);
    }
}
