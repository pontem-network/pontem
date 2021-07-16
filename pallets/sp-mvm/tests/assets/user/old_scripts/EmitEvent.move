script {
    use 0x1::Event;
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::EventProxy;

    fun emit_event(signer: &signer, val: u64) {
        EventProxy::emit_event(signer, val);
        Event::emit(signer, EventProxy::create_val(val));
    }
}
