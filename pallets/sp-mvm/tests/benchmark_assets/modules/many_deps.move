address 0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291 {
module StdImport {
    use 0x1::AccountLimits;
    use 0x1::DiemBlock;
    use 0x1::Event;
    use 0x1::FixedPoint32::FixedPoint32;
    use 0x1::PONT::PONT;
    use 0x1::Signer;
    use 0x1::DiemTimestamp;
    use 0x1::U256;
    use 0x1::Vector;
    use 0x1::DiemAccount::DiemAccount;

    struct T {
        t: DiemAccount,
        b: DiemBlock::BlockMetadata,
        f: FixedPoint32,
        p: PONT,
        tm: DiemTimestamp::CurrentTimeMicroseconds,
        u256: U256::U256,
        al: AccountLimits::AccountLimitMutationCapability,
    }

    struct R {}

    public fun foo(addr: &signer) {
        Signer::address_of(addr);
        let vec = Vector::empty<u8>();

        Event::publish_generator(addr);

        let event_handle = Event::new_event_handle(addr);
        Event::emit_event(
            &mut event_handle,
            vec
        );
        Event::destroy_handle(event_handle);
    }
}
}
