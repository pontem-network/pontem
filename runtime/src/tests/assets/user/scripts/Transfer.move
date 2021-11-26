script {
    use 0x1::DiemAccount;
    use 0x1::Signer;

    // Transfer from Bob to Alice.
    fun transfer<Token: key + store>(tc_account: signer, bob: signer, alice: address, to_move: u64) {
        // Create accounts.
        DiemAccount::create_parent_vasp_account<Token>(
            &tc_account,
            Signer::address_of(&bob),
            x"",
            x"626f62",
            true
        );

        DiemAccount::create_parent_vasp_account<Token>(
            &tc_account,
            alice,
            x"",
            x"616c696365",
            true
        );
        
        // Do transfers.
        let cap = DiemAccount::extract_withdraw_capability(&bob);
        DiemAccount::pay_from<Token>(&cap, alice, to_move, x"", x"");
        DiemAccount::restore_withdraw_capability(cap);
    }
}
