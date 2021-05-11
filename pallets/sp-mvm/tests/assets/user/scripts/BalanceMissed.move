script {
    use 0x1::Account;
    use 0x1::Coins;

    fun missed_native_balance(account: &signer) {
        Account::get_native_balance<Coins::BTC>(account);
    }
}
