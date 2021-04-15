use std::sync::Arc;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::{Block as BlockT}};
use sp_api::ProvideRuntimeApi;
use sp_mvm_rpc_runtime::MVMApiRuntime;
use frame_support::weights::Weight;

#[rpc]
pub trait MVMApiRpc<BlockHash> {
	#[rpc(name = "mvm_gasToWeight")]
	fn gas_to_weight(
		&self,
		at: Option<BlockHash>,
		gas: u64,
	) -> Result<Weight>;

	#[rpc(name = "mvm_weightToGas")]
	fn weight_to_gas(
		&self,
		at: Option<BlockHash>,
		weight: Weight,
	) -> Result<u64>;

	#[rpc(name= "mvm_estimateGasPublish")]
	fn estimate_gas_publish(
		&self,
		at: Option<BlockHash>,
		module_bc: Vec<u8>
	) -> Result<u64>;
}

pub struct MVMApi<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> MVMApi<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}


impl<C, Block> MVMApiRpc<
	<Block as BlockT>::Hash,
> for MVMApi<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: MVMApiRuntime<Block>,

{
	fn gas_to_weight(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		gas: u64,
	) -> Result<Weight> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

        let res = api.gas_to_weight(&at, gas);

        res.map_err(|e| RpcError {
			code: ErrorCode::ServerError(500),
			message: "Something went wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
    }

	fn weight_to_gas(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		weight: Weight,
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

        let res = api.weight_to_gas(&at, weight);

        res.map_err(|e| RpcError {
			code: ErrorCode::ServerError(500),
			message: "Something went wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
    }

	fn estimate_gas_publish(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		module_bc: Vec<u8>
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

        let res = api.estimate_gas_publish(&at, module_bc).map_err(|e| RpcError {
			code: ErrorCode::ServerError(500),
			message: "Something went wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})?;

		res.map_err(|e| RpcError {
			code: ErrorCode::ServerError(500),
			message: "Something went wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
