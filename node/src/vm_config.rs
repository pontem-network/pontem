use move_core_types::value::{serialize_values, MoveValue};
use diem_types::account_config;
use diem_types::chain_id::ChainId;

const MODULE_NAME: &[u8] = "Genesis".as_bytes();
const FUNC_NAME: &[u8] = "initialize".as_bytes();

// Build configuration to call initialize functions on standard library.
pub fn build() -> (Vec<u8>, Vec<u8>, Vec<Vec<u8>>) {
    let chain_id: ChainId = Default::default();
    let args = serialize_values(&vec![
        MoveValue::Signer(account_config::diem_root_address()), // dr_signer
        MoveValue::Signer(account_config::treasury_compliance_account_address()), // tr_signer
        MoveValue::vector_u8(account_config::diem_root_address().to_vec()), // dr_address
        MoveValue::vector_u8(account_config::treasury_compliance_account_address().to_vec()), // tr_address
        MoveValue::U8(chain_id.id()),
    ]);

    (MODULE_NAME.to_vec(), FUNC_NAME.to_vec(), args)
}
