use crate::{mock::*, helpers::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::{sr25519};
use frame_system::Config;
use sp_runtime::traits::StaticLookup;

#[test]
fn correct_error_for_unauthorised() {
	let users = get_users();
	let account1 = users[0].clone();
	let account2 = users[1].clone();
	let account3 = users[2].clone();
	let signer = account1.clone();
	let migration_vault_account = get_account_id_from_seed::<sr25519::Public>("MigrationVault");
	let migration_owner_account = account2.clone();

	new_test_ext(users, signer.clone(), 1000, migration_vault_account.clone(), migration_owner_account.clone()).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			Migration::migrate(RuntimeOrigin::signed(signer.clone()), account3.clone().into(), account3.clone(), 50),
			Error::<TestSuite>::Unauthorised
		);
	});
}

#[test]
fn migration_should_work_for_correct_values() {
	let users = get_users();
	let account1 = users[0].clone();
	let account2 = users[1].clone();
	let account3 = users[2].clone();
	let signer = account1.clone();
	let migration_vault_account = get_account_id_from_seed::<sr25519::Public>("MigrationVault");
	let migration_owner_account = account1.clone();
	let vault_total = 1000;
	let migrate_amount = 50;

	new_test_ext(users, signer.clone(), vault_total, migration_vault_account.clone(), migration_owner_account.clone()).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Dispatch a signed extrinsic.
		assert_ok!(Migration::migrate(RuntimeOrigin::signed(signer.clone()), account2.clone().into(), account3.clone(), migrate_amount));

		let token_id = Migration::get_token_id().unwrap();
		
		// Read pallet storage and assert an expected result.
		let vault_balance = Assets::balance(token_id, &migration_vault_account);
		let account_balance = Assets::balance(token_id, &account3);

		assert_eq!(vault_balance, vault_total - migrate_amount);
		assert_eq!(account_balance, migrate_amount);

		// Assert that the correct event was deposited
		System::assert_last_event(Event::BalanceMigrated { 
			amount: migrate_amount,
			from_vault: migration_vault_account,
			for_account: account2.clone().into(),
			to_account: account3.clone().into(),
			vault_balance_remained: vault_total - migrate_amount,
			account_balance_after: migrate_amount
		}.into());
	});
}

#[test]
fn double_migration_should_work_for_correct_values() {
	let users = get_users();
	let account1 = users[0].clone();
	let account2 = users[1].clone();
	let account3 = users[2].clone();
	let signer = account1.clone();
	let migration_vault_account = get_account_id_from_seed::<sr25519::Public>("MigrationVault");
	let migration_owner_account = account1.clone();
	let vault_total = 1000;
	let migrate_amount1= 50;
	let migrate_amount2= 50;

	new_test_ext(users, signer.clone(), vault_total, migration_vault_account.clone(), migration_owner_account.clone()).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Dispatch a signed extrinsic.
		assert_ok!(Migration::migrate(RuntimeOrigin::signed(signer.clone()), account2.clone().into(), account3.clone(), migrate_amount1));

		let token_id = Migration::get_token_id().unwrap();
		
		// Read pallet storage and assert an expected result.
		let mut vault_balance = Assets::balance(token_id, &migration_vault_account);
		let mut account_balance = Assets::balance(token_id, &account3);

		assert_eq!(vault_balance, vault_total - migrate_amount1);
		assert_eq!(account_balance, migrate_amount1);

		// Assert that the correct event was deposited
		System::assert_last_event(Event::BalanceMigrated { 
			amount: migrate_amount1,
			from_vault: migration_vault_account.clone(),
			for_account: account2.clone().into(),
			to_account: account3.clone().into(),
			vault_balance_remained: vault_total - migrate_amount1,
			account_balance_after: migrate_amount1
		}.into());

		// Dispatch a signed extrinsic.
		assert_ok!(Migration::migrate(RuntimeOrigin::signed(signer.clone()), account2.clone().into(), account3.clone(), migrate_amount2));

		// Read pallet storage and assert an expected result.
		vault_balance = Assets::balance(token_id, &migration_vault_account);
		account_balance = Assets::balance(token_id, &account3);

		assert_eq!(vault_balance, vault_total - migrate_amount1 - migrate_amount2);
		assert_eq!(account_balance, migrate_amount1 + migrate_amount2);

		// Assert that the correct event was deposited
		System::assert_last_event(Event::BalanceMigrated { 
			amount: migrate_amount2,
			from_vault: migration_vault_account,
			for_account: account2.clone().into(),
			to_account: account3.clone().into(),
			vault_balance_remained: vault_total - migrate_amount1 - migrate_amount2,
			account_balance_after: migrate_amount1 + migrate_amount2
		}.into());
	});
}
