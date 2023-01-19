#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod helpers;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	#[allow(unused)]
	use frame_support::{ debug };
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{ Currency };
	use frame_support::sp_runtime::traits::{ IdentifyAccount, Convert};
	use frame_support::sp_runtime::FixedPointOperand;
    use frame_support::traits::tokens::{Balance};
	use frame_support::traits::fungibles::{Inspect, Transfer, Create, Mutate};
	use crate::weights::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[cfg(feature = "runtime-benchmarks")]
	pub trait BenchmarkHelper<AssetIdParameter> {
		fn create_asset_id_parameter(id: u32) -> AssetIdParameter;
	}
	#[cfg(feature = "runtime-benchmarks")]
	impl<AssetIdParameter: From<u32>> BenchmarkHelper<AssetIdParameter> for () {
		fn create_asset_id_parameter(id: u32) -> AssetIdParameter {
			id.into()
		}
	}

	type BalanceOf<T> = <<T as Config>::LocalToken as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type AssetBalanceOf<T> = <T as Config>::AssetBalance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type LocalToken: Currency<Self::AccountId>;
		type TokenId: Member + Parameter + Copy + MaybeSerializeDeserialize + MaxEncodedLen;
		
		// Two-way conversion between asset and currency balances
		type AssetToCurrencyBalance: Convert<Self::AssetBalance, BalanceOf<Self>>;
		type CurrencyToAssetBalance: Convert<BalanceOf<Self>, Self::AssetBalance>;
		
		type AssetBalance: Balance
			+ FixedPointOperand
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ TypeInfo;
		type Assets: Inspect<Self::AccountId, AssetId = Self::TokenId, Balance = Self::AssetBalance> + Transfer<Self::AccountId> + Create<Self::AccountId> + Mutate<Self::AccountId>;
		type MigrationVaultAccount: IdentifyAccount;
		type MigrationOwner: IdentifyAccount;
		type WeightInfo: crate::weights::WeightInfo;

		// Helper trait for benchmarks.
		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper: BenchmarkHelper<Self::AssetIdParameter>;

		#[cfg(feature = "runtime-benchmarks")]
		type AssetIdParameter: Parameter
		+ Copy
		+ From<Self::TokenId>
		+ Into<Self::TokenId>
		+ MaxEncodedLen;
	}

	pub trait ConfigHelper: Config {
        fn currency_to_asset(curr_balance: BalanceOf<Self>) -> AssetBalanceOf<Self>;
        fn asset_to_currency(asset_balance: AssetBalanceOf<Self>) -> BalanceOf<Self>;
    }

    impl<T: Config> ConfigHelper for T {
        #[inline(always)]
        fn currency_to_asset(curr_balance: BalanceOf<Self>) -> AssetBalanceOf<Self> {
            Self::CurrencyToAssetBalance::convert(curr_balance)
        }

        #[inline(always)]
        fn asset_to_currency(asset_balance: AssetBalanceOf<Self>) -> BalanceOf<Self> {
            Self::AssetToCurrencyBalance::convert(asset_balance)
        }
    }

	#[pallet::storage]
	#[pallet::getter(fn get_vault)]
	pub type MigrationVaultAccount<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn get_owner)]
	pub type MigrationOwner<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn get_token_id)]
	pub type TokenId<T: Config> = StorageValue<_, T::TokenId>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub migration_vault_account: Option<T::AccountId>,
		pub migration_owner: Option<T::AccountId>,
		pub asset_id: Option<T::TokenId>
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { 
				migration_vault_account: Option::None,
				migration_owner: Option::None,
				asset_id: Option::None
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let tmp1 = self.migration_vault_account.clone();
			match tmp1 {
				Some(a) => <MigrationVaultAccount<T>>::put(a),
				None => debug(&Error::<T>::MigrationVaultAccountNoValue),
			}

			let tmp2 = self.migration_owner.clone();
			match tmp2 {
				Some(a) => <MigrationOwner<T>>::put(a),
				None => debug(&Error::<T>::MigrationOwnerNoValue),
			}

			let tmp3 = self.asset_id.clone();
			match tmp3 {
				Some(a) => <TokenId<T>>::put(a),
				None => debug(&Error::<T>::AssetIdNoValue),
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BalanceMigrated { 
			amount: BalanceOf<T>,
			from_vault: T::AccountId, 
			for_account: [u8; 32], 
			to_account: T::AccountId,
			vault_balance_remained: BalanceOf<T>,
			account_balance_after: BalanceOf<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		Unauthorised,
		MigrationVaultAccountNoValue,
		MigrationOwnerNoValue,
		AssetIdNoValue
	}
	
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::migrate())]
		pub fn migrate(origin: OriginFor<T>, for_account: [u8; 32], account_to_credit: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!( <MigrationVaultAccount<T>>::exists(),
				Error::<T>::MigrationVaultAccountNoValue
			);

			ensure!(<MigrationOwner<T>>::exists(),
				Error::<T>::MigrationOwnerNoValue
			);

			ensure!(<TokenId<T>>::exists(),
				Error::<T>::AssetIdNoValue
			);

			let owner = <MigrationOwner<T>>::get().unwrap();
			let migration_account = <MigrationVaultAccount<T>>::get().unwrap();
			let asset_id = <TokenId<T>>::get().unwrap();
			let migration_amount = T::currency_to_asset(amount);

			#[cfg(feature = "std")]
			{
				debug(&format!("The vault is: {migration_account:?}"));
				debug(&format!("The owner is: {owner:?}"));
				debug(&format!("The sender is: {who:?}"));
			}

			ensure!(
				owner == who,
				Error::<T>::Unauthorised
			);
			
			T::Assets::transfer(asset_id, &migration_account, &account_to_credit, migration_amount, true)?;

			let vault_balance = T::Assets::balance(asset_id, &migration_account);
			let account_balance = T::Assets::balance(asset_id, &account_to_credit);

			#[cfg(feature = "std")]
			{
				debug(&format!("Vault balance: {vault_balance:?}"));
				debug(&format!("Account balance: {account_balance:?}"));
			}
			
			Self::deposit_event(Event::BalanceMigrated {
				amount: amount,
				from_vault: migration_account, 
				for_account: for_account, 
				to_account: account_to_credit,
				vault_balance_remained: T::asset_to_currency(vault_balance),
				account_balance_after: T::asset_to_currency(account_balance),
			});
			Ok(())
		}
	}
}
