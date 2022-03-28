/// VM configuration for tests.
use move_vm::genesis::GenesisConfig;

/// Module to call to initialize genesis.
const MODULE_NAME: &[u8] = "Genesis".as_bytes();

/// Function to call to initialize genesis.
const FUNC_NAME: &[u8] = "initialize".as_bytes();

/// Build configuration to call initialize functions on Standard Library.
pub fn build() -> (Vec<u8>, Vec<u8>, Vec<Vec<u8>>) {
    // We use standard arguments.
    let genesis: GenesisConfig = Default::default();

    (
        MODULE_NAME.to_vec(),
        FUNC_NAME.to_vec(),
        genesis.init_func_config.unwrap().args,
    )
}
