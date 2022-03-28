script {
    use PontemFramework::PontAccount;
    use PontemFramework::KSM::KSM;
    use Std::Signer;
    use UserTests::Store;

    fun store_token_balance(account: signer) {
        let balance = PontAccount::balance<KSM>(Signer::address_of(&account));
        Store::store_u64(&account, balance);
    }
}
