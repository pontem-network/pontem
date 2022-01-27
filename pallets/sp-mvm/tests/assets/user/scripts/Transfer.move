script {
    use PontemFramework::PontAccount;
    use Std::Signer;
    use UserTests::Store;

    // Transfer from Bob to Alice.
    fun transfer<TokenType>(bob: signer, alice: address, to_move: u64) {
        PontAccount::pay_from<TokenType>(&bob, alice, to_move);

        let balance = PontAccount::balance<TokenType>(Signer::address_of(&bob));
        Store::store_u64(&bob, balance);
    }
}
