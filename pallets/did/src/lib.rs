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
		type DidIdentifier: Parameter + MaxEncodedLen + From<Self::AccountId> + Into<Self::AccountId>;

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
			document: Document<T>,
		},
		DidUpdated {
			did: DidIdentifierOf<T>,
			document: Document<T>,
		},
		DidForcedUpdated {
			did: DidIdentifierOf<T>,
			document: Document<T>,
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
		// Unable to find DID from DidIdentifier
		DidNotFound,
		/// Insufficient funds for DID deposit
		InsufficientBalance,
		/// The origin was not the controller of the DID
		NotController,
		/// Service already exist in the DID document
		ServiceAlreadyInDid,
		/// The service hash was not found in the DID
		ServiceNotInDid,
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
			let did = T::DidIdentifier::from(origin.clone());

			// Ensure origin has enough balance for DidDeposit.
			// ED does not need to be accounted for as the deposit is reserved.
			ensure!(T::Currency::free_balance(&origin) >= T::DidDeposit::get(), Error::<T>::InsufficientBalance);

			// Check DID does not exist yet
			ensure!(!Did::<T>::contains_key(did.clone()), Error::<T>::DidAlreadyExists);

			// Add assertion method
			let maybe_assertion_method = if let Some(assertion) = assertion {
				Some( AssertionMethod::<T> { controller: assertion } )
			} else { None };

			// Add services
			let maybe_services = if let Some(services) = services {
				Some ( Self::try_insert_services(services)? )
			} else { None };

			// Build Document
			let document = Document {
				controller,
				authentication: AuthenticationMethod { controller: authentication },
				assertion_method: maybe_assertion_method,
				services: maybe_services,
			};

			// Reserve did deposit. Should not fail as free_balance has been checked
			T::Currency::reserve(&origin, T::DidDeposit::get());

			// Store new DID
			Did::<T>::insert(did.clone(), document.clone());

			// Event
			Self::deposit_event(Event::DidCreated{
				did,
				document
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller: Option<DidIdentifierOf<T>>,
			authentication: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<Service<T>, T::MaxServices>>,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			// Try to mutate the document associated with the given DID identifier.
			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				// If document exists get mutable reference. Otherwise return error
				let did_doc = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;
				// ensure that the caller is the controller of the DID
				Self::ensure_controller(caller, &did_doc);

				// Try to update the document
				Self::try_update_did(
					did_doc,
					did.clone(),
					controller,
					authentication,
					assertion,
					services
				)?;

				// Emit event
				Self::deposit_event(Event::DidUpdated{
					did,
					document: did_doc.clone()
				});
				Ok(())
			})
		}

		#[pallet::weight(1000000)]
		pub fn force_update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller:Option<DidIdentifierOf<T>>,
			authentication: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<Service<T>, T::MaxServices>>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;

			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				let did_doc = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;

				Self::try_update_did(
					did_doc,
					did.clone(),
					controller,
					authentication,
					assertion,
					services
				)?;
				Self::deposit_event(Event::DidForcedUpdated{
					did,
					document: did_doc.clone()
				});
				Ok(())
			})
		}

		#[pallet::weight(1000000)]
		pub fn remove_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;

			//
			Did::<T>::try_mutate_exists(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				let did_doc = maybe_doc.take().ok_or(Error::<T>::DidNotFound)?;
				// ensure that the caller is the controller of the DID
				Self::ensure_controller(controller, &did_doc);

				T::Currency::unreserve(&did.clone().into(), T::DidDeposit::get());
				Self::deposit_event(Event::DidRemoved{
					did,
				});
				Ok(())
			})
		}

		#[pallet::weight(1000000)]
		pub fn force_remove_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;

			Did::<T>::try_mutate_exists(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				let did_doc = maybe_doc.take().ok_or(Error::<T>::DidNotFound)?;
				T::Currency::unreserve(&did.clone().into(), T::DidDeposit::get());
				Self::deposit_event(Event::DidForcedRemoved{
					did,
				});
				Ok(())
			})
		}

		#[pallet::weight(1000000)]
		pub fn add_did_service(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			service: Service<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;

			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				let did_doc = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;

				Self::ensure_controller(controller, &did_doc);

				let service_hash = Self::do_insert_service(service);

				// if services exists, insert new service in sorted location
				// else insert a new BoundedVec with the new service in the did_doc
				if let Some(mut services) = did_doc.services.as_mut() {
					let location = services.binary_search(&service_hash).err().ok_or(Error::<T>::ServiceAlreadyInDid)?;
					services.try_insert(location, service_hash.clone())
					.map_err(|_| Error::<T>::TooManyServicesInDid)?;
				} else {
					let mut services: BoundedVec<KeyIdOf<T>, T::MaxServices> = BoundedVec::default();
					// Only fails if `MaxServices` is 0
					services.try_push(service_hash).map_err(|_| Error::<T>::TooManyServicesInDid)?;
					did_doc.services = Some(services);
				};

				Self::deposit_event(Event::DidServiceAdded{
					did,
					id: service_hash,
				});
				Ok(())
			})
		}

		#[pallet::weight(1000000)]
		pub fn remove_did_service(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			service: Service<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;

			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				let did_doc = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;
				// ensure that the caller is the controller of the DID
				Self::ensure_controller(controller, &did_doc);

				let deleted_hash = if let Some(mut services) = did_doc.services.as_mut() {
					let service_hash = T::Hashing::hash_of(&service);
					let location = services.binary_search(&service_hash).ok().ok_or(Error::<T>::ServiceNotInDid)?;
					services.remove(location)
				} else {
					return Err(Error::<T>::ServiceNotInDid)?
				};

				Self::deposit_event(Event::DidServiceRemoved{
					did,
					id: deleted_hash
				});
				Ok(())
			})
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
	fn try_insert_services(services: BoundedVec<Service<T>, T::MaxServices>) -> Result<BoundedVec<KeyIdOf<T>, T::MaxServices>, DispatchError> {
		let mut hashed_services: BoundedVec<KeyIdOf<T>, T::MaxServices> = BoundedVec::default();
		for service in services {
			let service_hash = Self::do_insert_service(service);
			let location = hashed_services.binary_search(&service_hash).err().ok_or(Error::<T>::ServiceAlreadyInDid)?;
			hashed_services
				.try_insert(location, service_hash.clone())
				.map_err(|_| Error::<T>::TooManyServicesInDid)?;
		}
		Ok(hashed_services)
	}

	/// Insert a single service into storage
	fn do_insert_service(service: Service<T>) -> KeyIdOf<T> {
		let service_hash = T::Hashing::hash_of(&service);

		if !<Services<T>>::contains_key(service_hash) {
			<Services<T>>::insert(service_hash, service);
			service_hash
		} else {
			// service_hash already exists in storage,
			service_hash
		 }
	}

	/// Updates `did_doc` with specified fields. Inserting services may fail.
	/// Only modifies `did_doc` and does not write to storage.
	/// Should be used along with `try_mutate`.
	fn try_update_did(
		did_doc: &mut Document<T>,
		did: DidIdentifierOf<T>,
		controller: Option<DidIdentifierOf<T>>,
		authentication: Option<T::AuthenticationAddress>,
		assertion: Option<T::AssertionAddress>,
		services: Option<BoundedVec<Service<T>, T::MaxServices>>
	) -> Result<(), DispatchError> {

		// If present, update `controller`
		if let Some(controller) = controller {
			did_doc.controller = controller;
		}

		// If present, update `authentication` method
		if let Some(authentication) = authentication {
			// `authentication` is a required field, the struct will already exist
			did_doc.authentication.controller = authentication;
		}

		// If present, update `assertion_method`
		if let Some(assertion) = assertion {
			// `assertion_method` is optional, so ensure struct is created
			did_doc.assertion_method = Some (
				AssertionMethod {
					controller: assertion
				}
			);
		}

		// If present, update the `services` BoundedVec
		if let Some(services) = services {
			// Try to insert services to `Service` storage and
			// save new vec to the document.
			did_doc.services = Some( Self::try_insert_services(services)? );
		}

		Ok(())
	}

	/// Increment stored service consumers count. Lookup by service hash
	pub fn inc_consumers(service_hash: KeyIdOf<T>) -> Result<(), DispatchError> {
		Services::<T>::try_mutate(service_hash, |service| -> Result<(), DispatchError> {
			let service = service.as_mut().ok_or(Error::<T>::ServiceNotInDid)?;
			//TODO
			Ok(())
		})
	}

	/// Decrement stored service consumers count. Lookup by service hash.
	/// Will remove service if `consumers` becomes 0
	pub fn dec_consumers_with_removal(service_hash: KeyIdOf<T>) -> Result<(), DispatchError> {
		Services::<T>::try_mutate_exists(service_hash, |service| -> Result<(), DispatchError> {
			let service = service.as_mut().ok_or(Error::<T>::ServiceNotInDid)?;
			//TODO
			Ok(())
		})
	}

	/// Ensures that `who` is the controller of the did document
	fn ensure_controller(who: T::AccountId, did_doc: &Document<T>) -> DispatchResult {
		ensure!(did_doc.controller == T::DidIdentifier::from(who), Error::<T>::NotController);
		Ok(())
	}
}
