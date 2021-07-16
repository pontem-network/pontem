script {
//    use 0x1::Account;
    use 0x1::PONT;
//    use 0x1::Pontem;
    use 0x1::DiemAccount;
    use 0x1::Diem;
    use 0x1::Signer;
    use 0x1::CoreAddresses;

    fun test_balance_transfer(root_acc: signer, alice: signer, bob: address, amount: u64, register_coin: bool) {
        let alice_addr = Signer::address_of(&alice);
        assert(DiemAccount::balance(alice_addr) >= amount, 1);
        assert(amount > 3, 2);
//        assert(Account::get_native_balance<PONT::T>(alice) >= amount, 1);

        if (register_coin) {
            Diem::register_currency<PONT::PONT>(root_acc, );
//            Pontem::register_coin<PONT::T>(b"PONT", 2);
        };

        Diem::register_currency()
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
