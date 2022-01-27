script {
    use UserTests::EventProxy;

    fun emit_event(acc: signer, val: u64) {
        EventProxy::emit_event(&acc, val);
    }
}
