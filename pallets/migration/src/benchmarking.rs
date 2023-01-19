//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Migration;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::inherent::Vec;
use sp_core::Encode;
use frame_support::traits::fungibles::{Create, Mutate};

const SEED: u32 = 0;
const VAULT_INDEX: u32 = 4;
const TOKEN_ID: u32 = 1;

fn default_asset_id<T: Config>() -> T::AssetIdParameter {
	T::BenchmarkHelper::create_asset_id_parameter(TOKEN_ID)
}

fn create_default_asset<T: Config>() -> (T::AssetIdParameter, T::AccountId) {
	let asset_id = default_asset_id::<T>();
	let caller: T::AccountId = whitelisted_caller();
	T::Assets::create(
		asset_id.into(),
		caller.clone(),
		true,
		1u32.into(),
	);
	(asset_id, caller.clone())
}

fn create_default_minted_asset<T: Config>(
	who: T::AccountId,
	amount: u32,
) {
	let (asset_id, _owner) = create_default_asset::<T>();

	assert!(T::Assets::mint_into(
		asset_id.into(),
		&who.clone(),
		amount.into(),
	)
	.is_ok());
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn account_to_bytes<AccountId>(account: &AccountId) -> [u8; 32]
	where AccountId: Encode,
{
	let account_vec = account.encode();
	let mut bytes = [0u8; 32];
	bytes.copy_from_slice(&account_vec);
	bytes
}

benchmarks! {
	migrate {
		let caller: T::AccountId = whitelisted_caller();

		let account1 = account::<T::AccountId>("Alice", 1, SEED);
		let account2 = account::<T::AccountId>("Bob", 2, SEED);
		let account3 = account::<T::AccountId>("Charlie", 3, SEED);

		let mut users = Vec::new();
		users.push(account1.clone());
		users.push(account2.clone());
		users.push(account3.clone());

		let signer = account1.clone();
		let migration_vault_account = account::<T::AccountId>("MigrationVault", VAULT_INDEX, SEED);
		let migration_owner_account = account1.clone();
		let vault_total = 1_000_000;
		let migrate_amount = 100;

		let for_account = account_to_bytes(&account2);

		<MigrationVaultAccount<T>>::put(migration_vault_account.clone());
		<MigrationOwner<T>>::put(caller.clone());
		<TokenId<T>>::put(default_asset_id::<T>().into());

		create_default_minted_asset::<T>(migration_vault_account.clone(), vault_total);
	}: _(RawOrigin::Signed(caller.clone()), for_account, account3.clone(), migrate_amount.into())
	verify {
		assert_last_event::<T>(
			Event::BalanceMigrated { 
				amount: migrate_amount.into(),
				from_vault: migration_vault_account,
				for_account: for_account,
				to_account: account3.clone().into(),
				vault_balance_remained: (vault_total - migrate_amount).into(),
				account_balance_after: migrate_amount.into()
			}.into()
		);
	}

	impl_benchmark_test_suite!(Migration, crate::mock::new_default_ext(), crate::mock::TestSuite);
}
