script {
    use 0x1::DiemAccount;
    use 0x1::PONT::PONT;
    use 0x1::Signer;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    // Transfer from Bob to Alice.
    fun transfer(tc_account: signer, bob: signer, alice: address, to_move: u64) {
        // Create accounts.
        DiemAccount::create_parent_vasp_account<PONT>(
            &tc_account,
            Signer::address_of(&bob),
            x"",
            x"626f62",
            true
        );

        DiemAccount::create_parent_vasp_account<PONT>(
            &tc_account,
            alice,
            x"",
            x"616c696365",
            true
        );
        
        // Do transfers.
        let cap = DiemAccount::extract_withdraw_capability(&bob);
        DiemAccount::pay_from<PONT>(&cap, alice, to_move, x"", x"");
        DiemAccount::restore_withdraw_capability(cap);

        let balance = DiemAccount::balance<PONT>(Signer::address_of(&bob));
        Store::store_u64(&bob, balance);
    }
}
