script {
    use 0x1::DiemAccount;
    use 0x1::PONT::PONT;
    use {{sender}}::BankPONT;

    fun deposit_bank(sender: signer, amount: u64) {
        // Withdraw PONT tokens from sender account.
        let pont_tokens = DiemAccount::pnt_withdraw<PONT>(&sender, amount);

        BankPONT::deposit(&sender, pont_tokens);        
    }
}
 