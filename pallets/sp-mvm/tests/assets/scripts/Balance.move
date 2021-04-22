script {
    use 0x1::Account;
    use 0x1::PONT;
    use 0x1::Pontem;

    fun test_balance_transfer(alice: &signer, bob: address, amount: u128) {
        assert(Account::get_native_balance<PONT::T>(alice) >= amount, 1);
        assert(amount > 3, 2);

        let ponts = Account::deposit_native<PONT::T>(alice, amount - 3);
        Account::deposit(alice, bob, ponts);

        let ponts_1 = Account::deposit_native<PONT::T>(alice, 1);
        let ponts_2 = Account::deposit_native<PONT::T>(alice, 1);
        let ponts_3 = Account::deposit_native<PONT::T>(alice, 1);

        let ponts_1 = Pontem::join(ponts_1, ponts_2);
        let ponts = Pontem::join(ponts_1, ponts_3);

        Account::deposit(alice, bob, ponts);
    }
}
