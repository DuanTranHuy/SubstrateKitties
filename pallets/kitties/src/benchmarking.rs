//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	set_price{ let s in 0 .. 10000000;
		let caller: T::AccountId = whitelisted_caller();
		let _ = Kitties::<T>::mint(&caller, None, None);
		let kitty_ids = Kitties::<T>::kitties_owned(&caller);
	}: _(RawOrigin::Signed(caller), kitty_ids[0], Some(s.into()))

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
