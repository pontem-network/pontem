script {
    use PontemFramework::PontAccount;
    use RuntimeTests::Bank;

    fun deposit_bank<TokenType>(sender: signer, amount: u64) {
        // Withdraw TokenType tokens from sender account.
        let tokens = PontAccount::withdraw<TokenType>(&sender, amount);

        Bank::deposit(&sender, tokens);
    }
}
 