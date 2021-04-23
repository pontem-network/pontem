script {
    use 0x1::Account;
    use 0x1::Pontem;
    use 0x1::PONT;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_native_withdraw(account: &signer, register_coin: bool) {
        if (register_coin) {
            Pontem::register_coin<PONT::T>(b"PONT", 2);
        };

        let balance = Account::get_native_balance<PONT::T>(account);
        let ponts = Account::deposit_native<PONT::T>(account, balance / 2);

        Account::withdraw_native(account, ponts);

        let balance = Account::get_native_balance<PONT::T>(account);
        Store::store_u128(account, balance);
    }
}
