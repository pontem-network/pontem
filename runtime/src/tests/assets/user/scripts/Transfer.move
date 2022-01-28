script {
    use PontemFramework::PontAccount;

    /// Transfer TokenType tokens from Bob to Alice.
    fun transfer<TokenType>(bob: signer, alice: address, to_move: u64) {
        PontAccount::pay_from<TokenType>(&bob, alice, to_move);
    }
}
