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

mod types;
mod verification;
mod errors;

use sp_std::{prelude::*, fmt::Debug};
use sp_core::H160;
use frame_support::{
	BoundedVec,
	dispatch::{DispatchResult, Dispatchable, GetDispatchInfo, PostDispatchInfo},
	ensure,
	storage::types::StorageMap,
	traits::{Currency, Get, OnUnbalanced, WithdrawReasons, ReservableCurrency},
	pallet_prelude::DispatchError,
	Parameter,
};
use sp_runtime::traits::{Hash};
use crate::verification::{DidSignature};
use crate::types::{AssertionMethod, AuthenticationMethod, Document, IssuerInfo, Service};

pub use pallet::*;
use verification::{DidVerifiableIdentifier};

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

	/// Type for a DID subject identifier.
	pub type DidIdentifierOf<T> = <T as Config>::DidIdentifier;

	/// Type for valid Credentials.
	pub type CredentialOf<T> = BoundedVec<u8, <T as Config>::MaxString>;

	/// Type for valid Credentials.
	pub type HashOf<T> = BoundedVec<u8, <T as Config>::MaxHash>;

	/// Type for a Service hash identifier.
	pub type KeyIdOf<T> = <T as frame_system::Config>::Hash;

	/// Type for Watr account identifier.
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	/// Type for a block number.
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub(crate) type CurrencyOf<T> = <T as Config>::Currency;
	pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::NegativeImbalance;

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

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Type for a DID subject identifier.
		type DidIdentifier: Parameter + MaxEncodedLen + From<Self::AccountId>;

		/// Type for the authentication method used by a DID.
		type AuthenticationAddress: Parameter + DidVerifiableIdentifier + MaxEncodedLen;

		/// Type for the assertion method used by an Issuer DID.
		type AssertionAddress: Parameter + DidVerifiableIdentifier + MaxEncodedLen;

		/// The amount held on deposit for a DID creation
		#[pallet::constant]
		type DidDeposit: Get<BalanceOf<Self>>;

		/// The maximum number of Service per ID.
		#[pallet::constant]
		type MaxServices: Get<u32>;

		/// The maximum length of a String
		#[pallet::constant]
		type MaxString: Get<u32>;

		/// The maximum length of a Hash
		#[pallet::constant]
		type MaxHash: Get<u32>;

		/// The maximum Credential types.
		#[pallet::constant]
		type MaxCredentialsTypes: Get<u32>;

		// Origin for priviledged actions
		type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	/// DID Resolver
	#[pallet::storage]
	#[pallet::getter(fn dids)]
	pub type Did<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DidIdentifierOf<T>,
		Document<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn services)]
	pub type Services<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		KeyIdOf<T>,
		Service<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn issuers)]
	pub type Issuers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DidIdentifierOf<T>,
		IssuerInfo,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn credential_types)]
	// list of valid Credentials types
	pub(super) type CredentialsTypes<T: Config> = StorageValue<
		_,
		BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		DidCreated {
			did: DidIdentifierOf<T>,
			// document: Document<T>,
		},
		DidUpdated {
			did: DidIdentifierOf<T>,
			// document: Document<T>,
		},
		DidForcedUpdated {
			did: DidIdentifierOf<T>,
			// document: Document<T>,
		},
		DidRemoved {
			did: DidIdentifierOf<T>,
		},
		DidForcedRemoved {
			did: DidIdentifierOf<T>,
		},
		DidServiceAdded { did: DidIdentifierOf<T>, id: T::Hash },
		DidServiceRemoved { did: DidIdentifierOf<T>, id: T::Hash },
		CredentialsIssued {
			issuer: DidIdentifierOf<T>,
			did: DidIdentifierOf<T>,
			credentials: Vec<CredentialOf<T>>,
			storage_hash: HashOf<T>,
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Unable to create DID that already exists
		DidAlreadyExists,
		/// Service already exist in the DID document
		ServiceAlreadyInDid,
		/// The maximum number of Services in the DID has been excedeed
		TooManyServicesInDid
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(1000000)]
		pub fn create_did(
			origin: OriginFor<T>,
			controller: DidIdentifierOf<T>,
			authentication: T::AuthenticationAddress,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<Service<T>, T::MaxServices>>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let did = T::DidIdentifier::from(origin);

			// Check DID does not exist yet
			ensure!(!Did::<T>::contains_key(did.clone()), Error::<T>::DidAlreadyExists);

			// Add assertion method
			let maybe_assertion_method = if let Some(assertion) = assertion {
				Some( AssertionMethod::<T> { controller: assertion } )
			} else { None };

			// Add services
			let maybe_services = if let Some(services) = services {
				let mut hashed_services: BoundedVec<KeyIdOf<T>, T::MaxServices> = BoundedVec::default();
				for service in services {
					if let Some(service_hash) = Self::try_add_service(service) {
						let location = hashed_services.binary_search(&service_hash).err().ok_or(Error::<T>::ServiceAlreadyInDid)?;
						hashed_services
							.try_insert(location, service_hash.clone())
							.map_err(|_| Error::<T>::TooManyServicesInDid)?;
					}
				}
				Some(hashed_services)
			} else { None };

			// Build Document
			let document = Document {
				controller,
				authentication: AuthenticationMethod { controller: authentication },
				assertion_method: maybe_assertion_method,
				services: maybe_services,
			};

			// Reserve currency
			// TODO: Check origin has enough balance & reserve DidDeposit

			// Store new DID
			Did::<T>::insert(did.clone(), document);

			// Event
			Self::deposit_event(Event::DidCreated{
				did,
				// document
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller:Option<DidIdentifierOf<T>>,
			authentication: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<Service<T>, T::MaxServices>>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;
			Self::deposit_event(Event::DidUpdated{
				did,
				// document
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn force_update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller:Option<DidIdentifierOf<T>>,
			authenticaton: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<Service<T>, T::MaxServices>>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::DidForcedUpdated{
				did,
				// document
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn remove_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;
			Self::deposit_event(Event::DidRemoved{
				did,
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn force_remove_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::DidForcedRemoved{
				did,
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn add_did_service(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			service: Service<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;
			let service_hash = T::Hashing::hash_of(&service);
			Self::deposit_event(Event::DidServiceAdded{
				did,
				id: service_hash,
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn remove_did_service(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			service: Service<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;
			let service_hash = T::Hashing::hash_of(&service);
			Self::deposit_event(Event::DidServiceRemoved{
				did,
				id: service_hash
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn issue_credentials(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			credentials: Vec<CredentialOf<T>>,
			storage_hash: HashOf<T>,
		) -> DispatchResult {
			let issuer = ensure_signed(origin)?;
			Self::deposit_event(Event::CredentialsIssued{
				issuer: T::DidIdentifier::from(issuer),
				did,
				credentials,
				storage_hash,
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn add_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			// Add issuer to database with status Active
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn revoke_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			// Change status to Revoked
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn remove_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Called by the user to de
			T::GovernanceOrigin::ensure_origin(origin)?;
			// Remove issuer from storage
			// Should be also called when a Issuer DID is deleted()
			// because of that prob create a do_delete_issuer() pallet helper method
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn add_credential_type(origin: OriginFor<T>, credential: CredentialOf<T>) -> DispatchResult {
 			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn remove_credential_type(origin: OriginFor<T>, credential: CredentialOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			Ok(())
		}
	}
}


impl<T: Config> Pallet<T> {
	fn try_add_service(service: Service<T>) -> Option<KeyIdOf<T>> {
		let service_hash = T::Hashing::hash_of(&service);

		if !<Services<T>>::contains_key(service_hash) {
			<Services<T>>::insert(service_hash, service);
			Some(service_hash)
		} else { None }
	}
}
