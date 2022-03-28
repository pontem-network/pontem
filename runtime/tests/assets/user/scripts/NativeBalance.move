script {
    use PontemFramework::PontAccount;
    use PontemFramework::NOX::NOX;
    use Std::Signer;
    use UserTests::Store;

    fun store_native_balance(account: signer) {
        let balance = PontAccount::balance<NOX>(Signer::address_of(&account));
        Store::store_u64(&account, balance);
    }
}
