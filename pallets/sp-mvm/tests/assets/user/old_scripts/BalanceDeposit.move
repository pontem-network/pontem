script {
//    use 0x1::Account;
    use 0x1::Signer;
//    use 0x1::Pontem;
    use 0x1::PONT;
    use 0x1::Diem;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_native_deposit(account: &signer, register_coin: bool) {
        if (register_coin) {
            Diem::register_currency<PONT::PONT>(
                &root_acc,
                FixedPoint32::create_from_raw_value(1),
                true,
                1,
                1,
                b"pont");
            //            Pontem::register_coin<PONT::T>(b"PONT", 2);
        };

        let balance = Account::get_native_balance<PONT::T>(account);
        let ponts = Account::deposit_native<PONT::T>(account, balance / 2);
        Account::deposit(account, Signer::address_of(account), ponts);

        let balance = Account::get_native_balance<PONT::T>(account);
        Store::store_u128(account, balance);
    }
}
