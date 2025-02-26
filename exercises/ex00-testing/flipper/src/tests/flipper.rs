use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

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
		assert_noop!(
			Flipper::set_value(Origin::signed(ALICE), true),
			Error::<TestRuntime>::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext().execute_with(|| {
		// todo!("Ensure the good behaviour of the flip_value() function.")
		let first = Flipper::set_value(Origin::signed(ALICE), false);
		assert_eq!(Flipper::value(), Some(false));
		assert_ok!(first);

		let second = Flipper::flip_value(Origin::signed(ALICE));
		assert_eq!(Flipper::value(), Some(true));
		assert_ok!(second);
	});
}

#[test]
fn flip_value_ko() {
	new_test_ext().execute_with(|| {
		// todo!("write a scenario that triggers an error in flip_value()")
		assert_noop!(
			Flipper::flip_value(Origin::signed(ALICE)),
			Error::<TestRuntime>::NoneValue
		);
	});
}
