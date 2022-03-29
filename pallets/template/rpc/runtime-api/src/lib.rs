#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;


sp_api::decl_runtime_apis! {
	pub trait SumStorageApi
	{
	fn get_sum() -> u32	;
	}
}