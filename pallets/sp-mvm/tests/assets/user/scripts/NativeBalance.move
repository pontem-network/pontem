script {
    use PontemFramework::PontAccount;
    use PontemFramework::PONT::PONT;
    use Std::Signer;
    use UserTests::Store;

    fun store_native_balance(account: signer) {
        let balance = PontAccount::balance<PONT>(Signer::address_of(&account));
        Store::store_u64(&account, balance);
    }
}
