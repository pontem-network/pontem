use std::sync::Arc;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::{Block as BlockT}};
use sp_api::ProvideRuntimeApi;
use sp_mvm_rpc_runtime::TestAPIRuntime;

#[rpc]
pub trait TestAPI<BlockHash> {
	#[rpc(name = "mvm_test")]
	fn test(
		&self,
		at: Option<BlockHash>
	) -> Result<u64>;
}

pub struct Test<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Test<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}


impl<C, Block> TestAPI<
	<Block as BlockT>::Hash,
> for Test<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: TestAPIRuntime<Block>,

{
	fn test(
		&self,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

        let res = api.test(&at);

        res.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
    }
}
