script {
    use 0x1::DiemAccount;
    use {{sender}}::Bank;

    fun deposit_bank<Token: key + store>(sender: signer, amount: u64) {
        // Withdraw PONT tokens from sender account.
        let tokens = DiemAccount::pnt_withdraw<Token>(&sender, amount);

        Bank::deposit(&sender, tokens);        
    }
}
 