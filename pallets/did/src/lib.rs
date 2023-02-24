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
	types::{AssertionMethod, AuthenticationMethod, Document, IssuerInfo, IssuerStatus, Service, ServiceInfo},
	verification::DidSignature,
};
use frame_support::{
	dispatch::{DispatchResult, Dispatchable, GetDispatchInfo, PostDispatchInfo},
	ensure,
	pallet_prelude::DispatchError,
	storage::types::StorageMap,
	traits::{Currency, EnsureOrigin, Get, OnUnbalanced, ReservableCurrency, WithdrawReasons},
	BoundedVec, Parameter,
};
use frame_system::{ensure_signed, pallet_prelude::OriginFor};
use sp_core::H160;
use sp_runtime::traits::Hash;
use sp_runtime::ArithmeticError;
use sp_std::{fmt::Debug, prelude::*};

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

	/// Type for a DID subject identifier.
	pub type DidIdentifierOf<T> = <T as Config>::DidIdentifier;

	/// Type for valid Credentials.
	pub type CredentialOf<T> = BoundedVec<u8, <T as Config>::MaxString>;

	/// Type for valid Credentials.
	pub type HashOf<T> = BoundedVec<u8, <T as Config>::MaxHash>;

	/// Type for a Service key identifier.
	pub type KeyIdOf<T> = <T as frame_system::Config>::Hash;

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

	/// Type for a BoundedVec of `Service` keys
	pub type ServiceKeysOf<T> = BoundedVec<KeyIdOf<T>, <T as Config>::MaxServices>;

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
		type DidIdentifier: Parameter
			+ MaxEncodedLen
			+ From<Self::AccountId>
			+ Into<Self::AccountId>;

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
	pub type Did<T: Config> = StorageMap<_, Blake2_128Concat, DidIdentifierOf<T>, Document<T>>;

	#[pallet::storage]
	#[pallet::getter(fn services)]
	pub type Services<T: Config> = StorageMap<_, Blake2_128Concat, KeyIdOf<T>, Service<T>>;

	#[pallet::storage]
	#[pallet::getter(fn issuers)]
	pub type Issuers<T: Config> = StorageMap<_, Blake2_128Concat, DidIdentifierOf<T>, IssuerInfo>;

	#[pallet::storage]
	#[pallet::getter(fn credential_types)]
	// list of valid Credentials types
	pub(super) type CredentialsTypes<T: Config> =
		StorageValue<_, BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CredentialTypeAdded {
			credential: CredentialOf<T>,
		},
		CredentialTypeRemoved {
			credential: CredentialOf<T>,
		},
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
		DidServicesAdded {
			did: DidIdentifierOf<T>,
			new_services: ServiceKeysOf<T>,
		},
		DidServicesRemoved {
			did: DidIdentifierOf<T>,
			removed_services: ServiceKeysOf<T>,
		},
		CredentialsIssued {
			issuer: DidIdentifierOf<T>,
			did: DidIdentifierOf<T>,
			credentials: Vec<CredentialOf<T>>,
			storage_key: HashOf<T>,
		},
		IssuerDeleted {
			issuer: DidIdentifierOf<T>,
		},
		IssuerStatusReactived {
			issuer: DidIdentifierOf<T>,
		},
		IssuerStatusActive {
			issuer: DidIdentifierOf<T>,
		},
		IssuerStatusRevoked {
			issuer: DidIdentifierOf<T>,
		},
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

		/// Unable to create DID that already exists
		DidAlreadyExists,
		// Unable to find DID from DidIdentifier
		DidNotFound,
		/// The origin was not the controller of the DID
		NotController,
		/// Service already exist in the DID document
		ServiceAlreadyInDid,
		/// The service key was not found in the DID
		ServiceNotInDid,
		/// Too many references to a service. Not likely to happen
		TooManyServiceConsumers,
		/// The maximum number of Services in the DID has been excedeed
		TooManyServicesInDid,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// #[pallet::call_index(0)]
		#[pallet::weight(1000000)]
		pub fn create_did(
			origin: OriginFor<T>,
			controller: DidIdentifierOf<T>,
			authentication: T::AuthenticationAddress,
			assertion: Option<T::AssertionAddress>,
			services: BoundedVec<ServiceInfo<T>, T::MaxServices>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let did = T::DidIdentifier::from(origin.clone());

			// Reserve did deposit.
			// If user does not have enough balance returns `InsufficientBalance`
			T::Currency::reserve(&origin, T::DidDeposit::get())?;

			// Check that DID does not exist yet
			ensure!(!Did::<T>::contains_key(did.clone()), Error::<T>::DidAlreadyExists);

			// Add assertion method
			let maybe_assertion_method =
				assertion.map(|assertion| AssertionMethod::<T> { controller: assertion });

			// Add services. Transpose converts from Option<Result> to Result<Option>
			// allowing for proper error handling.
			// let services_keyes =
			// 	services.map(|s| Self::do_add_did_services(s, None)).transpose()?;

			let services_keys = Self::do_add_did_services(services, None)?;

			// Build Document
			let document = Document {
				controller,
				authentication: AuthenticationMethod { controller: authentication },
				assertion_method: maybe_assertion_method,
				services: services_keys,
			};

			// Store new DID
			Did::<T>::insert(did.clone(), document.clone());

			// Event
			Self::deposit_event(Event::DidCreated { did, document });
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller: Option<DidIdentifierOf<T>>,
			authentication: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<ServiceInfo<T>, T::MaxServices>>,
		) -> DispatchResult {
			let document = Self::do_update_did(
				origin,
				did.clone(),
				controller,
				authentication,
				assertion,
				services,
			)?;
			Self::deposit_event(Event::DidUpdated { did, document });
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn force_update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller: Option<DidIdentifierOf<T>>,
			authentication: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<ServiceInfo<T>, T::MaxServices>>,
		) -> DispatchResult {
			T::GovernanceOrigin::ensure_origin(origin.clone())?;
			let document = Self::do_update_did(
				origin,
				did.clone(),
				controller,
				authentication,
				assertion,
				services,
			)?;
			Self::deposit_event(Event::DidForcedUpdated { did, document });
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn remove_did(origin: OriginFor<T>, did: DidIdentifierOf<T>) -> DispatchResult {
			Self::do_remove_did(origin, did.clone())?;
			Self::deposit_event(Event::DidRemoved { did });
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn force_remove_did(origin: OriginFor<T>, did: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin.clone())?;
			Self::do_remove_did(origin, did.clone())?;
			Self::deposit_event(Event::DidForcedRemoved { did });
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn add_did_services(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			services: BoundedVec<ServiceInfo<T>, T::MaxServices>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;

			// Try to mutate document
			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				let document = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;

				Self::ensure_controller(controller, &document)?;

				// Insert new services. If document.services is Some, then add services, otherwise create new vec
				let services_keys =
					Self::do_add_did_services(services, Some(document.services.clone()))?;
				document.services = services_keys.clone();

				Self::deposit_event(Event::DidServicesAdded { did, new_services: services_keys });
				Ok(())
			})
		}

		#[pallet::weight(1000000)]
		pub fn remove_did_services(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			services_keys: ServiceKeysOf<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;

			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
				let document = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;
				// ensure that the caller is the controller of the DID
				Self::ensure_controller(controller, &document)?;

				let removed_services = Self::do_remove_did_services(&mut document.services, services_keys)?;

				Self::deposit_event(Event::DidServicesRemoved { did, removed_services });
				Ok(())
			})
		}

		#[pallet::weight(1000000)]
		pub fn issue_credentials(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			credentials: Vec<CredentialOf<T>>,
			storage_key: HashOf<T>,
		) -> DispatchResult {
			let issuer = ensure_signed(origin)?;
			Self::deposit_event(Event::CredentialsIssued {
				issuer: T::DidIdentifier::from(issuer),
				did,
				credentials,
				storage_key,
			});
			Ok(())
		}

		// #[pallet::call_index(1)]
		#[pallet::weight(1000000)]
		pub fn add_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			ensure!(!Issuers::<T>::contains_key(&issuer), Error::<T>::IssuerAlreadyExists);
			// Add issuer to storage with status Active
			Issuers::<T>::insert(issuer.clone(), IssuerInfo { status: IssuerStatus::Active });
			Self::deposit_event(Event::IssuerStatusActive { issuer });
			Ok(())
		}

		// #[pallet::call_index(2)]
		#[pallet::weight(1000000)]
		pub fn revoke_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;

			// Change issuer status to Revoked
			Issuers::<T>::try_mutate(issuer.clone(), |maybe_info| -> DispatchResult {
				let mut info = maybe_info.as_mut().ok_or(Error::<T>::IssuerDoesNotExist)?;
				ensure!(
					*info == IssuerInfo { status: IssuerStatus::Active },
					Error::<T>::IssuerNotActive
				);
				*info = IssuerInfo { status: IssuerStatus::Revoked };
				Self::deposit_event(Event::IssuerStatusRevoked { issuer });
				Ok(())
			})
		}

		// #[pallet::call_index(3)]
		#[pallet::weight(1000000)]
		pub fn reactivate_issuer(
			origin: OriginFor<T>,
			issuer: DidIdentifierOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;

			// Change issuer status to Active
			Issuers::<T>::try_mutate(issuer.clone(), |maybe_info| -> DispatchResult {
				let mut info = maybe_info.as_mut().ok_or(Error::<T>::IssuerDoesNotExist)?;
				ensure!(
					*info == IssuerInfo { status: IssuerStatus::Revoked },
					Error::<T>::IssuerNotRevoked
				);
				*info = IssuerInfo { status: IssuerStatus::Active };
				Self::deposit_event(Event::IssuerStatusReactived { issuer });
				Ok(())
			})
		}

		// #[pallet::call_index(4)]
		#[pallet::weight(1000000)]
		pub fn remove_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin.clone())?;
			Self::do_remove_issuer(origin, issuer)?;
			Ok(())
		}

		// #[pallet::call_index(5)]
		#[pallet::weight(1000000)]
		pub fn add_credential_type(
			origin: OriginFor<T>,
			credential: CredentialOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			let mut credentials_types = CredentialsTypes::<T>::get();
			let pos = credentials_types
				.binary_search(&credential)
				.err()
				.ok_or(Error::<T>::CredentialAlreadyAdded)?;
			credentials_types
				.try_insert(pos, credential.clone())
				.map_err(|_| Error::<T>::MaxCredentials)?;

			CredentialsTypes::<T>::put(credentials_types);
			Self::deposit_event(Event::CredentialTypeAdded { credential });
			Ok(())
		}

		// #[pallet::call_index(6)]
		#[pallet::weight(1000000)]
		pub fn remove_credential_type(
			origin: OriginFor<T>,
			credential: CredentialOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			let mut credentials_types = CredentialsTypes::<T>::get();
			let pos = credentials_types
				.binary_search(&credential)
				.ok()
				.ok_or(Error::<T>::CredentialDoesNotExist)?;
			credentials_types.remove(pos);
			CredentialsTypes::<T>::put(credentials_types);
			Self::deposit_event(Event::CredentialTypeRemoved { credential });
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_remove_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
		T::GovernanceOrigin::ensure_origin(origin)?;
		Issuers::<T>::try_mutate_exists(issuer.clone(), |maybe_info| -> DispatchResult {
			let mut info = maybe_info.as_mut().ok_or(Error::<T>::IssuerDoesNotExist)?;
			ensure!(
				*info == IssuerInfo { status: IssuerStatus::Revoked },
				Error::<T>::IssuerNotRevoked
			);
			// Remove issuer from storage
			*maybe_info = None;
			Self::deposit_event(Event::IssuerDeleted { issuer });
			Ok(())
		})
	}

	fn do_add_did_services(
		services: BoundedVec<ServiceInfo<T>, T::MaxServices>,
		existing_services: Option<ServiceKeysOf<T>>,
	) -> Result<ServiceKeysOf<T>, DispatchError> {
		// if existing_services is `Some` use it to insert into, otherwise create a new BoundedVec
		let mut services_keys = existing_services.unwrap_or_default();

		for service in services {
			let service_key = Self::do_add_service(service)?;
			let pos = services_keys
				.binary_search(&service_key)
				.err()
				.ok_or(Error::<T>::ServiceAlreadyInDid)?;
			services_keys
				.try_insert(pos, service_key.clone())
				.map_err(|_| Error::<T>::TooManyServicesInDid)?;
		}
		Ok(services_keys)
	}

	fn do_remove_did_services(document_services: &mut ServiceKeysOf<T>, keys_to_remove: ServiceKeysOf<T>) -> Result<ServiceKeysOf<T>, DispatchError> {
		// Save the removed service keys
		let mut removed_services = BoundedVec::default();

		// Iterate over each service and remove it from the document
		for service_key in keys_to_remove {
			let pos = document_services
				.binary_search(&service_key)
				.ok()
				.ok_or(Error::<T>::ServiceNotInDid)?;
			let deleted_key = document_services.remove(pos);

			removed_services.try_push(deleted_key);
		}
		Ok(removed_services)
	}

	/// Insert a single service into storage
	fn do_add_service(service: ServiceInfo<T>) -> Result<KeyIdOf<T>, DispatchError> {
		let service_key = T::Hashing::hash_of(&service);

		// if the service exists, increment its consumers, otherwise insert a new service
		if let Some(mut existing_service) = Services::<T>::get(service_key.clone()) {
			// `inc_consumers` may overflow, so handle it just in case
			existing_service.inc_consumers().map_err(|_| Error::<T>::TooManyServiceConsumers)?;
		} else {
			<Services<T>>::insert(service_key, Service::<T>::new(service));
		}

		Ok(service_key)
	}

	/// Updates `document` with specified fields. Inserting services may fail.
	fn do_update_did(
		origin: OriginFor<T>,
		did: DidIdentifierOf<T>,
		controller: Option<DidIdentifierOf<T>>,
		authentication: Option<T::AuthenticationAddress>,
		assertion: Option<T::AssertionAddress>,
		services: Option<BoundedVec<ServiceInfo<T>, T::MaxServices>>,
	) -> Result<Document<T>, DispatchError> {
		Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<Document<T>, DispatchError> {
			let document = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;

			// Check if origin is either governance or controller
			Self::ensure_governance_or_controller(origin, &document)?;

			// If present, update `controller`
			if let Some(controller) = controller {
				document.controller = controller;
			}

			// If present, update `authentication` method
			if let Some(authentication) = authentication {
				// `authentication` is a required field, the struct will already exist
				document.authentication.controller = authentication;
			}

			// If present, update `assertion_method`
			if let Some(assertion) = assertion {
				// `assertion_method` is optional, so ensure struct is created
				document.assertion_method = Some(AssertionMethod { controller: assertion });
			}

			// If present, update the `services` BoundedVec
			if let Some(new_services) = services {
				// Try to insert services to `Service` storage and
				// save new vec to the document -- overwriting old services.
				document.services = Self::do_add_did_services(new_services, None)?;
			}

			Ok(document.clone())
		})
	}

	pub fn do_remove_did(origin: OriginFor<T>, did: DidIdentifierOf<T>) -> DispatchResult {
		Did::<T>::try_mutate(did.clone(), |maybe_doc| -> Result<(), DispatchError> {
			let document = maybe_doc.take().ok_or(Error::<T>::DidNotFound)?;

			// Check if origin is either governance or controller
			Self::ensure_governance_or_controller(origin, &document)?;
			// TODO - do_remove_services()

			// If DID belongs to Issuer
			// Self::do_remove_issuer(origin, issuer)?;
			T::Currency::unreserve(&did.clone().into(), T::DidDeposit::get());
			Ok(())
		})
	}

	/// Increment stored service consumers count. Lookup by service key
	pub fn inc_consumers(service_key: KeyIdOf<T>) -> Result<(), DispatchError> {
		Services::<T>::try_mutate(service_key, |service| -> Result<(), DispatchError> {
			let service = service.as_mut().ok_or(Error::<T>::ServiceNotInDid)?;
			//TODO
			Ok(())
		})
	}

	/// Decrement stored service consumers count. Lookup by service key.
	/// Will remove service if `consumers` becomes 0
	pub fn dec_consumers_with_removal(service_key: KeyIdOf<T>) -> Result<(), DispatchError> {
		Services::<T>::try_mutate_exists(service_key, |service| -> Result<(), DispatchError> {
			let service = service.as_mut().ok_or(Error::<T>::ServiceNotInDid)?;
			//TODO
			Ok(())
		})
	}

	/// Ensures that `who` is the controller of the did document
	fn ensure_controller(who: T::AccountId, document: &Document<T>) -> DispatchResult {
		ensure!(document.controller == T::DidIdentifier::from(who), Error::<T>::NotController);
		Ok(())
	}

	// Check if origin is either governance or controller
	fn ensure_governance_or_controller(
		origin: OriginFor<T>,
		document: &Document<T>,
	) -> DispatchResult {
		T::GovernanceOrigin::ensure_origin(origin.clone())
			.map_or(Self::ensure_controller(ensure_signed(origin)?, document), |_| Ok(()))
	}
}
