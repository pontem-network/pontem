script {
    use 0x1::Account;
    use 0x1::PONT;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_native_balance(account: &signer) {
        let balance = Account::get_native_balance<PONT::T>(account);
        Store::store_u128(account, balance);
    }
}
