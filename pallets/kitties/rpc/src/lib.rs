
pub use self::gen_client::Client as TransactionPaymentClient;
use codec::{Codec, Decode};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, MaybeDisplay},
};
use std::sync::Arc;
use pallet_kitties_rpc_runtime_api::KittyApi;

#[rpc]
pub trait KittyApi<BlockHash> {
	#[rpc(name = "getKitty_count")]
    fn get_kitty_count(&self, at: Option<BlockHash>) -> Result<u64>;
	#[rpc(name = "get_kitty")]
    fn get_kitty_count(&self, at: Option<BlockHash>) -> Result<Kitty>;
}

/// A struct that implements the [`TransactionPaymentApi`].
pub struct KittyApi<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> KittyApi<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block> SumstorageApi<<Block as BlockT>::Hash>
	for KittyApi<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: KittyApi<Block>,
{
	fn get_kitty_count(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<u32> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		// let encoded_len = encoded_xt.len() as u32;
        let result_api = api.get_kitty_count(&at);
		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

}