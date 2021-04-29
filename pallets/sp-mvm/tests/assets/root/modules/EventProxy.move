module EventProxy {
    use 0x01::Event;

    struct U64 { val: u64 }

    public fun emit_event(addr: &signer, val: u64) {
        Event::emit<U64>(addr, U64 { val })
    }

    public fun create_val(val: u64): U64 {
        U64 { val }
    }
}