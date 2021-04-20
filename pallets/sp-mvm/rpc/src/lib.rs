use std::sync::Arc;
use std::convert::From;
use codec::{self, Codec};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT},
};
use sp_api::ProvideRuntimeApi;
use sp_mvm_rpc_runtime::{MVMApiRuntime, types::MVMApiEstimation};
use frame_support::weights::Weight;
use serde::{Serialize, Deserialize};

// Estimation struct with serde.
#[derive(Serialize, Deserialize)]
pub struct Estimation {
    pub gas_used: u64,
    pub status_code: u64,
}

impl From<MVMApiEstimation> for Estimation {
    fn from(e: MVMApiEstimation) -> Self {
        Self {
            gas_used: e.gas_used,
            status_code: e.status_code,
        }
    }
}

// RPC calls.
#[rpc]
pub trait MVMApiRpc<BlockHash, AccountId> {
    #[rpc(name = "mvm_gasToWeight")]
    fn gas_to_weight(&self, at: Option<BlockHash>, gas: u64) -> Result<Weight>;

    #[rpc(name = "mvm_weightToGas")]
    fn weight_to_gas(&self, at: Option<BlockHash>, weight: Weight) -> Result<u64>;

    #[rpc(name = "mvm_estimateGasPublish")]
    fn estimate_gas_publish(
        &self,
        at: Option<BlockHash>,
        account: AccountId,
        module_bc: Vec<u8>,
        gas_limit: u64,
    ) -> Result<Estimation>;

    #[rpc(name = "mvm_estimateGasExecute")]
    fn estimate_gas_execute(
        &self,
        at: Option<BlockHash>,
        account: AccountId,
        tx_bc: Vec<u8>,
        gas_limit: u64,
    ) -> Result<Estimation>;
}

pub struct MVMApi<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> MVMApi<C, P> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId> MVMApiRpc<<Block as BlockT>::Hash, AccountId> for MVMApi<C, Block>
where
    Block: BlockT,
    AccountId: Clone + std::fmt::Display + Codec,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: MVMApiRuntime<Block, AccountId>,
{
    fn gas_to_weight(&self, at: Option<<Block as BlockT>::Hash>, gas: u64) -> Result<Weight> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api.gas_to_weight(&at, gas);

        res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during requesting Runtime API".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn weight_to_gas(&self, at: Option<<Block as BlockT>::Hash>, weight: Weight) -> Result<u64> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api.weight_to_gas(&at, weight);

        res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during requesting Runtime API".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn estimate_gas_publish(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        account: AccountId,
        module_bc: Vec<u8>,
        gas_limit: u64,
    ) -> Result<Estimation> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api
            .estimate_gas_publish(&at, account, module_bc, gas_limit)
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Error during requesting Runtime API".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

        let mvm_estimation = res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during publishing module for estimation".into(),
            data: Some(format!("{:?}", e).into()),
        })?;

        Ok(Estimation::from(mvm_estimation))
    }

    fn estimate_gas_execute(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        account: AccountId,
        tx_bc: Vec<u8>,
        gas_limit: u64,
    ) -> Result<Estimation> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api
            .estimate_gas_execute(&at, account, tx_bc, gas_limit)
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Error during requesting Runtime API".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

        let mvm_estimation = res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during script execution for estimation".into(),
            data: Some(format!("{:?}", e).into()),
        })?;

        Ok(Estimation::from(mvm_estimation))
    }
}
