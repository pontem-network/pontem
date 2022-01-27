script {
    use PontemFramework::PontAccount;
    use UserTests::Bank;

    fun deposit_bank<TokenType: key + store>(sender: signer, amount: u64) {
        // Withdraw TokenType tokens from sender account.
        let tokens = PontAccount::withdraw<TokenType>(&sender, amount);

        Bank::deposit(&sender, tokens);        
    }
}
 