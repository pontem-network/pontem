script {
    use 0x1::Pontem;
    use 0x1::PONT;
    use 0x1::Coins;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;

    fun get_price_test(addr_for_eth_btc: &signer) {
        Pontem::register_coin<PONT::T>(b"PONT", 2);
        Pontem::register_coin<Coins::BTC>(b"BTC", 2);
        Store::store_u128(addr_for_eth_btc, Coins::get_price<Coins::BTC, PONT::T>());
    }
}
