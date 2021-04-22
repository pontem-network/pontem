script {
    use 0x1::Account;
    use 0x1::PONT;
    // use 0x1::Pontem;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_native_deposit(account: &signer) {
        let balance = Account::get_native_balance<PONT::T>(account);
        // let ponts = Account::deposit_native<PONT::T>(account, balance);
        // move_to<Pontem::T<PONT::T>>(account, ponts);
        Store::store_u128(account, balance);
    }
}
