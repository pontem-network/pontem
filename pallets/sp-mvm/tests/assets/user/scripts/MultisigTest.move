script {
    use 0x1::Signer;

    fun multisig_test(account1: signer, account2: signer) {
        // signers are sorted lexicographically
        assert(Signer::address_of(&account1) == @5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, 1); // Alice
        assert(Signer::address_of(&account2) == @5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty, 1); // Bob
    }
}
