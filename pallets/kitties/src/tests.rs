use crate::{mock::* , Error};
use frame_support::{assert_noop, assert_ok};

fn last_event() -> Event {
	<frame_system::Pallet<Test>>::events().pop().expect("Event expected").event
}

#[test]
fn should_working_create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_ok!(Kitties::create_kitty(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
	});
}

#[test]
fn cannot_set_price_for_not_kitty_owner() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Kitties::create_kitty(Origin::signed(1)));
		if let Event::Kitties(crate::Event::Created(_kitty_owned, kitty_hash)) = last_event() {
		 assert_noop!(Kitties::set_price(Origin::signed(5), kitty_hash, Some(100)), Error::<Test>::NotKittyOwner);
		}

	});
}

#[test]
fn cannot_set_price_for_unexisted_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_noop!(Kitties::set_price(Origin::signed(1), Default::default(), Some(100)), Error::<Test>::KittyNotExist);
		// Read pallet storage and assert an expected result.
	});
}


#[test]
fn can_set_price() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Kitties::create_kitty(Origin::signed(1)));
		if let Event::Kitties(crate::Event::Created(_kitty_owned, kitty_hash)) = last_event() {
		 assert_ok!(Kitties::set_price(Origin::signed(1), kitty_hash, Some(100)));
		}

	});
}

#[test]
fn can_transfer() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Kitties::create_kitty(Origin::signed(1)));
		if let Event::Kitties(crate::Event::Created(_kitty_owned, kitty_hash)) = last_event() {
		assert_ok!(Kitties::transfer(Origin::signed(1), 2u64, kitty_hash));
		}

	});
}

#[test]
fn cannot_transfer_to_self() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Kitties::create_kitty(Origin::signed(1)));
		if let Event::Kitties(crate::Event::Created(_kitty_owned, kitty_hash)) = last_event() {
		assert_noop!(Kitties::transfer(Origin::signed(1), 1u64, kitty_hash), Error::<Test>::TransferToSelf);
		}
	});
}

