//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Migration;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Migration, crate::mock::new_test_ext(), crate::mock::TestSuite);
}


/*

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Migration;
use frame_benchmarking::{
	benchmarks, account, benchmarks_instance_pallet, whitelist_account, whitelisted_caller,
};
use benchmarking::sp_runtime::AccountId32;
use sp_runtime::traits::TrailingZeroInput;
use frame_system::RawOrigin;
use frame_support::traits::{ Currency };
use frame_system::Config;
use sp_runtime::traits::StaticLookup;
use sp_runtime::{
	traits::{
		Verify, IdentifyAccount, Convert, Identity, IdentityLookup
	},
	MultiSignature,
};
use frame_support::inherent::Vec;
use frame_support::*;

const SEED: u32 = 0;

benchmarks! {
	migrate {
		//let s in 0 .. 1000;

		let account1 = account::<T::AccountId>("Alice", 1, SEED);
		let account2 = account::<T::AccountId>("Bob", 2, SEED);
		let account3 = account::<T::AccountId>("Charlie", 3, SEED);

		let mut users = Vec::new();
		users.push(account1.clone());
		users.push(account2.clone());
		users.push(account3.clone());

		let migrate_amount = 10;
		let signer = account1.clone();
		let migration_vault_account = account::<T::AccountId>("MigrationVault", 4, SEED);
		let migration_owner_account = account1.clone();
		let vault_total = 1_000_000;

		let caller: T::AccountId = whitelisted_caller();

	}: _(RawOrigin::Signed(caller), account2.clone().into(), account3, migrate_amount.into())
	verify {
		// Assert that the correct event was deposited
		assert_eq!(1, 2 / 2);
	}
}

*/
