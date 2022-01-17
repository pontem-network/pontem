script {
    use Std::Signer;

    fun multisig_test(account1: signer, account2: signer) {
        assert(Signer::address_of(&account1) == @Alice, 1); // Alice
        assert(Signer::address_of(&account2) == @Bob, 1); // Bob
    }
}
