script {
    use Std::Signer;

    fun as_root(root: signer) {
        assert!(Signer::address_of(&root) == @Root, 1);
    }
}
 