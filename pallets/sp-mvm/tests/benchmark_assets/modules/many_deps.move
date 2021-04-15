address 0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291 {
module StdImport {
    use 0x1::Account;
    use 0x1::Block;
    use 0x1::Coins;
    use 0x1::Compare;
    use 0x1::Debug;
    use 0x1::Event;
    use 0x1::FixedPoint32;
    use 0x1::Math;
    use 0x1::Offer;
    use 0x1::PONT;
    use 0x1::Pontem;
    use 0x1::Security;
    use 0x1::Signer;
    use 0x1::Time;
    use 0x1::U256;
    use 0x1::Vector;

    resource struct T {
        t: Account::T,
        b: Block::BlockMetadata,
        c: Coins::BTC,
        f: FixedPoint32::T,
        m: Math::Num,
        o: Offer::T<R>,
        p: PONT::T,
        pc: Pontem::T<Coins::BTC>,
        s: Security::Info,
        tm: Time::CurrentTimestamp,
        u256: U256::U256,
    }

    struct R {}

    public fun foo(addr: &signer) {
        Signer::address_of(addr);
        let vec = Vector::empty();
        Debug::print(&vec);
        Compare::cmp_lcs_bytes(&vec, &vec);
        Event::emit(addr, vec);
    }
}
}