script {
    use 0x1::DiemAccount;
    use 0x1::PONT::PONT;
    use 0x1::Signer;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun store_native_balance(account: signer) {
        let balance = DiemAccount::balance<PONT>(Signer::address_of(&account));
        Store::store_u64(&account, balance);
    }
}
