#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
pub use pallet_kitties::{Kitty, Config};

sp_api::decl_runtime_apis! {
	pub trait KittyApi
	{
	// fn get_kitty() -> Kitty<T>;
	fn get_kitty_count() -> u64	;
	}
}
