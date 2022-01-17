module BenchmarksAccount::StdImport {
    use PontemFramework::PontBlock;
    use Std::Event;
    use Std::FixedPoint32::FixedPoint32;
    use PontemFramework::PONT::PONT;
    use Std::Signer;
    use PontemFramework::PontTimestamp;
    use Std::Vector;
    use PontemFramework::PontAccount::PontAccount;
    use PontemFramework::Token;

    struct T {
        t: PontAccount,
        b: PontBlock::BlockMetadata,
        f: FixedPoint32,
        p: PONT,
        tm: PontTimestamp::CurrentTimeMicroseconds,
        u8_vec: vector<u8>,
        al: Token::MintCapability<PONT>,
    }

    struct R {}

    public fun foo(addr: &signer) {
        Signer::address_of(addr);
        let vec = Vector::empty<u8>();

        let event_handle = Event::new_event_handle(addr);
        Event::emit_event(
            &mut event_handle,
            vec
        );
        Event::destroy_handle(event_handle);
    }
}
