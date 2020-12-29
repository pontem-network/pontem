script {
    use 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48::Event;

    // fun emit_event(val: u64) {
    //     Event::write_to_event_store(b"GUID", 1, Event::new_u64(val));
    // }

    fun emit_event() {
        Event::write_to_event_store(b"GUID", 1, Event::new_u64(42));
    }
}
