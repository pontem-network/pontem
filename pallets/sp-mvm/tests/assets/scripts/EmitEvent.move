script {
    use 0x1::Event;

    fun emit_event(val: u64) {
        Event::write_to_event_store(b"GUID", 1, Event::new_u64(val));
    }
}
