// Copyright 2023 Watr Foundation
// This file is part of Watr.

// Watr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Watr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Watr.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod errors;
mod types;
mod verification;

use crate::{
	types::{IssuerInfo, IssuerStatus},
	verification::DidSignature,
};
use frame_support::{
	dispatch::{DispatchResult, Dispatchable, GetDispatchInfo, PostDispatchInfo},
	ensure,
	storage::types::StorageMap,
	traits::{Currency, Get, OnUnbalanced, ReservableCurrency, WithdrawReasons},
	BoundedVec, Parameter,
};
use sp_core::H160;
use sp_std::prelude::*;

pub use pallet::*;
use verification::DidVerifiableIdentifier;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	/// Reference to a payload of data of variable size.
	pub type Payload = [u8];

	// /// Type for a DID key identifier.
	// pub type KeyIdOf<T> = <T as frame_system::Config>::Hash;

	/// Type for a DID subject identifier.
	pub type DidIdentifierOf<T> = <T as Config>::DidIdentifier;

	/// Type for valid Credentials.
	pub type CredentialOf<T> = BoundedVec<u8, <T as Config>::MaxString>;

	/// Type for Watr account identifier.
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	/// Type for a block number.
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub(crate) type CurrencyOf<T> = <T as Config>::Currency;
	pub(crate) type NegativeImbalanceOf<T> =
		<<T as Config>::Currency as Currency<AccountIdOf<T>>>::NegativeImbalance;

	// /// Type for a runtime extrinsic callable under DID-based authorisation.
	// pub type DidCallableOf<T> = <T as Config>::RuntimeCall;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		/// Type for a DID subject identifier.
		type DidIdentifier: Parameter + DidVerifiableIdentifier + MaxEncodedLen;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The amount held on deposit for a DID creation
		#[pallet::constant]
		type DidDeposit: Get<BalanceOf<Self>>;

		// Origin for priviledged actions
		type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The maximum length of a service ID.
		#[pallet::constant]
		type MaxString: Get<u32>;

		/// The maximum Credential types.
		#[pallet::constant]
		type MaxCredentialsTypes: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn issuers)]
	pub type Issuers<T: Config> =
		StorageMap<_, Blake2_128Concat, DidIdentifierOf<T>, IssuerInfo, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn credential_types)]
	// list of valid Credentials types
	pub(super) type CredentialsTypes<T: Config> =
		StorageValue<_, BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CredentialTypeAdded { credential: CredentialOf<T> },
		CredentialTypeRemoved { credential: CredentialOf<T> },
		DidCreated { controller: AccountIdOf<T>, authenticator: H160, signature: DidSignature },
		IssuerDeleted { issuer: DidIdentifierOf<T> },
		IssuerStatusReactived { issuer: DidIdentifierOf<T> },
		IssuerStatusActive { issuer: DidIdentifierOf<T> },
		IssuerStatusRevoked { issuer: DidIdentifierOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		CredentialAlreadyAdded,
		CredentialDoesNotExist,
		IssuerAlreadyExists,
		IssuerDoesNotExist,
		IssuerNotActive,
		IssuerNotRevoked,
		MaxCredentials,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(1000000)]
		pub fn create_did(
			origin: OriginFor<T>,
			authenticator: H160, // Ethereum Address
			signature: DidSignature,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;
			Self::deposit_event(Event::DidCreated { controller, authenticator, signature });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(1000000)]
		pub fn add_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			ensure!(!Issuers::<T>::contains_key(&issuer), Error::<T>::IssuerAlreadyExists);
			// Add issuer to database with status Active
			Issuers::<T>::insert(issuer.clone(), IssuerInfo { status: IssuerStatus::Active });
			Self::deposit_event(Event::IssuerStatusActive { issuer });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(1000000)]
		pub fn revoke_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			ensure!(Issuers::<T>::contains_key(&issuer), Error::<T>::IssuerDoesNotExist);
			ensure!(
				Issuers::<T>::get(&issuer) == IssuerInfo { status: IssuerStatus::Active },
				Error::<T>::IssuerNotActive
			);
			// Change status to Revoked
			Issuers::<T>::insert(issuer.clone(), IssuerInfo { status: IssuerStatus::Revoked });
			Self::deposit_event(Event::IssuerStatusRevoked { issuer });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(1000000)]
		pub fn reactivate_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			ensure!(Issuers::<T>::contains_key(&issuer), Error::<T>::IssuerDoesNotExist);
			ensure!(
				Issuers::<T>::get(&issuer) == IssuerInfo { status: IssuerStatus::Revoked },
				Error::<T>::IssuerNotRevoked
			);
			// Change issuer's status from Revoked to Active
			Issuers::<T>::insert(issuer.clone(), IssuerInfo { status: IssuerStatus::Active });
			Self::deposit_event(Event::IssuerStatusReactived { issuer });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(1000000)]
		pub fn delete_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			ensure!(Issuers::<T>::contains_key(&issuer), Error::<T>::IssuerDoesNotExist);
			ensure!(
				Issuers::<T>::get(&issuer) == IssuerInfo { status: IssuerStatus::Revoked },
				Error::<T>::IssuerNotRevoked
			);
			// Remove issuer from storage
			Self::do_delete_issuer(issuer.clone())?;
			Self::deposit_event(Event::IssuerDeleted { issuer });
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(1000000)]
		pub fn add_credential_type(
			origin: OriginFor<T>,
			credential: CredentialOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			let mut credentials_types = CredentialsTypes::<T>::get();
			let pos = credentials_types.binary_search(&credential);
			ensure!(
				!pos.is_ok(),
				Error::<T>::CredentialAlreadyAdded
			);
			match pos {
				Err(pos) => {
					credentials_types
				.try_insert(pos, credential.clone())
				.map_err(|_| Error::<T>::MaxCredentials)?;
				()
				},
				Ok(_pos) => (),
			}
			CredentialsTypes::<T>::put(credentials_types);
			Self::deposit_event(Event::CredentialTypeAdded { credential });
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(1000000)]
		pub fn remove_credential_type(
			origin: OriginFor<T>,
			credential: CredentialOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			let mut credentials_types = CredentialsTypes::<T>::get();
			let pos = credentials_types.binary_search(&credential);
			ensure!(
				pos.is_ok(),
				Error::<T>::CredentialDoesNotExist
			);
			match pos {
				Ok(pos) => {
					credentials_types.remove(pos);
					()
				},
				Err(_pos) => (),
			}
			CredentialsTypes::<T>::put(credentials_types);
			Self::deposit_event(Event::CredentialTypeRemoved{ credential });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Delete issuer from Storage.
		pub fn do_delete_issuer(issuer: DidIdentifierOf<T>) -> DispatchResult {
			Issuers::<T>::remove(issuer);
			Ok(())
		}
	}
}
