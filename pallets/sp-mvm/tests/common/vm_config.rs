use move_vm::genesis::GenesisConfig;

const MODULE_NAME: &[u8] = "Genesis".as_bytes();
const FUNC_NAME: &[u8] = "initialize".as_bytes();

// Build configuration to call initialize functions on standard library.
pub fn build() -> (Vec<u8>, Vec<u8>, Vec<Vec<u8>>) {
    // We use standard arguments.
    let genesis: GenesisConfig = Default::default();

    (
        MODULE_NAME.to_vec(),
        FUNC_NAME.to_vec(),
        genesis.init_func_config.unwrap().args,
    )
}
