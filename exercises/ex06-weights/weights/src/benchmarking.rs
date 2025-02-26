use super::*;
// got the "file not included in module tree" warning ?
// look at the comment in Cargo.toml (in the feature section), and the one at the top of the lib.rs
// file
//
use crate::Pallet as Weights;
use frame_benchmarking::{account as benchmark_account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
	/////////////////////// Part 2 - benchmarks ///////////////////////

	//TODO: change this generic benchmark to benchmark the duplicate_and_store extrinsic
	duplicate_and_store {
		//this variable is a range, meaning the benchmark will be run with the different values of
		//s, to evaluate the weight of this specific parameter
		let s in 0 .. 10_000;
		// todo("change this range to something that makes sense for your benchmark");
		let count in 0 .. 5

		// let root = todo!("get the root origin, to sign our transactions");
		let root = ensure_signed(root)?;

		// Now that we have all the parameters we need for our extrinsic's benchmark, we can call
		// it:
	}: duplicate_and_store (root, count, s)
	verify {
		// Run some verifications here.
		// If something isn't right, the benchmark will throw an error and wont output values
		assert!(VecDup::<T>::get().unwrap().len() == s as usize);
	}

	/////////////////////// Part 3.A - conditional benchmarks ///////////////////////
	store_maybe_hashed_true {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		let root = ensure_signed(root)?;
		let data = vec![1; 1000];
		let hash = true;
	}: store_maybe_hashed(root, data, hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
	}

	store_maybe_hashed_false {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		let root = ensure_signed(root)?;
		let data = vec![1; 1000];
		let hash = false;
	}: store_maybe_hashed(root, data, hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
	}

	impl_benchmark_test_suite!(Weights, crate::mock::new_test_ext(), crate::mock::Test);
}
