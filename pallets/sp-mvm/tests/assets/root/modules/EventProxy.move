module EventProxy {
    use 0x01::Event;

    struct U64 has store, drop, copy { val: u64 }

    public fun emit_event(acc: &signer, val: u64) {
        let event_handle = Event::new_event_handle(acc);
        Event::emit_event(
            &mut event_handle,
            U64 { val }
        );
        Event::destroy_handle(event_handle);
    }
}