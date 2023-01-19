use crate::{self as pallet_migration};
#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking::{account, whitelisted_caller};
use frame_support::traits::{ConstU16, ConstU64, ConstU128, ConstU32, AsEnsureOriginWithArg, Currency};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, Identity, AccountIdLookup},
	BuildStorage
};
use sp_core::sr25519::Public;
use frame_system::{EnsureSigned};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestSuite>;
type Block = frame_system::mocking::MockBlock<TestSuite>;
type AccountId = crate::helpers::AccountId;

const TOKEN_ID: u32 = 1;
#[cfg(feature = "runtime-benchmarks")]
const SEED: u32 = 0;
#[cfg(feature = "runtime-benchmarks")]
const VAULT_INDEX: u32 = 4;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestSuite where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Migration: pallet_migration,
		Balances: pallet_balances,
		Assets:  pallet_assets,
	}
);

impl frame_system::Config for TestSuite {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_migration::Config for TestSuite {
	type TokenId = u32;
	type RuntimeEvent = RuntimeEvent;
	type AssetToCurrencyBalance = Identity;
	type CurrencyToAssetBalance = Identity;
	type LocalToken = Balances;
	type Assets = Assets;
	type AssetBalance = <pallet_balances::Pallet<TestSuite> as Currency<AccountId>>::Balance;
	type MigrationVaultAccount = Public;
	type MigrationOwner = Public;
	type WeightInfo = ();

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	#[cfg(feature = "runtime-benchmarks")]
	type AssetIdParameter = codec::Compact<u32>;
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 1;
/// Balance of an account.
pub type Balance = u128;

impl pallet_balances::Config for TestSuite {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<TestSuite>;
}

impl pallet_assets::Config for TestSuite {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = ConstU128<1>;
	type AssetAccountDeposit = ConstU128<1>;
	type MetadataDepositBase = ConstU128<1>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<TestSuite>;
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
}

#[cfg(feature = "runtime-benchmarks")]
pub fn new_default_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<TestSuite>().unwrap().into()
}

#[cfg(feature = "runtime-benchmarks")]
pub fn new_benchmark_ext() -> sp_io::TestExternalities {
	let caller: AccountId = whitelisted_caller();

	let account1 = account::<AccountId>("Alice", 1, SEED);
	let account2 = account::<AccountId>("Bob", 2, SEED);
	let account3 = account::<AccountId>("Charlie", 3, SEED);

	let mut users = Vec::new();
	users.push(account1.clone());
	users.push(account2.clone());
	users.push(account3.clone());

	let migration_vault_account = account::<AccountId>("MigrationVault", VAULT_INDEX, SEED);
	let migration_owner_account = caller.clone();
	let vault_total = 1_000_000;

	new_test_ext(users, caller, vault_total, migration_vault_account.clone(), migration_owner_account.clone())
}

pub fn new_test_ext(users: Vec<AccountId>, root_key: AccountId, vault_total: u128, migration_vault_account: AccountId, migration_owner_account: AccountId) -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<TestSuite>().unwrap();

	GenesisConfig {
		assets: AssetsConfig {
			assets: vec!((TOKEN_ID, root_key.clone(), true, 1)),
			/// Genesis metadata: id, name, symbol, decimals
			metadata: vec!((TOKEN_ID, "Jur token".as_bytes().to_vec(), "JUR".as_bytes().to_vec(), 12)),
			/// Genesis accounts: id, account_id, balance
			accounts: vec!((TOKEN_ID, migration_vault_account.clone(), vault_total)),

		},
		balances: BalancesConfig {
			balances: 
				users.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		migration: MigrationConfig {
			migration_vault_account: Some(migration_vault_account),
			migration_owner: Some(migration_owner_account),
			asset_id: Some(TOKEN_ID)
		},
		..Default::default()
	}
	.assimilate_storage(&mut storage)
	.unwrap();
	
	let mut externalities = sp_io::TestExternalities::new(storage);
	externalities.execute_with(|| System::set_block_number(1));
	externalities
}