use super::mock::*;
use crate::{pallet, Error, Config};
use frame_support::{assert_noop, assert_ok, assert_err};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(ALICE), false));
		assert_eq!(Flipper::value(), Some(false));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		// todo!("Verify if the function returns the expected error.")
		let _ = Flipper::set_value(Origin::signed(ALICE), true);
		assert_noop!(Flipper::set_value(Origin::signed(ALICE), true), Error::<TestRuntime>::AlreadySet);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext()
		.execute_with(|| todo!("Ensure the good behaviour of the flip_value() function."));
}

#[test]
fn flip_value_ko() {
	new_test_ext()
		.execute_with(|| todo!("write a scenario that triggers an error in flip_value()"));
}
