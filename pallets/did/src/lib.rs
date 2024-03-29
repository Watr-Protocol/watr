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
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod types;

use crate::types::{
	AssertionMethod, AuthenticationMethod, CredentialInfo, Document, IssuerInfo, IssuerStatus,
	Service, ServiceInfo, ServicesWitness,
};
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	pallet_prelude::DispatchError,
	storage::types::StorageMap,
	traits::{Currency, EnsureOrigin, Get, ReservableCurrency},
	BoundedVec, Parameter,
};
use frame_system::{ensure_signed, pallet_prelude::OriginFor};
use sp_core::{H160, H256};
use sp_runtime::{traits::Hash, ArithmeticError};
use sp_std::prelude::*;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::GetDispatchInfo, pallet_prelude::*, traits::UnfilteredDispatchable,
	};
	use frame_system::pallet_prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	/// Type for a DID subject identifier.
	pub type DidIdentifierOf<T> = <T as Config>::DidIdentifier;

	/// Type for valid Credentials.
	pub type CredentialOf<T> = BoundedVec<u8, <T as Config>::MaxCredentialTypeLength>;

	/// Type for valid Credentials.
	pub type HashOf<T> = BoundedVec<u8, <T as Config>::MaxHash>;

	/// Type for a Service key identifier.
	pub type KeyIdOf<T> = <T as frame_system::Config>::Hash;

	/// Type for Watr account identifier.
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	/// Type for a block number.
	pub type BlockNumberOf<T> = <HeaderFor<T> as sp_runtime::traits::Header>::Number;

	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;

	pub(crate) type CurrencyOf<T> = <T as Config>::Currency;

	/// Type for a BoundedVec of `Service` keys
	pub type ServiceKeysOf<T> = BoundedVec<KeyIdOf<T>, <T as Config>::MaxServices>;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
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
		type AuthenticationAddress: Parameter + MaxEncodedLen + From<H160> + From<H256>;

		/// Type for the assertion method used by an Issuer DID.
		type AssertionAddress: Parameter + MaxEncodedLen + From<H160> + From<H256>;

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

		/// The maximum length of a CredentialType
		#[pallet::constant]
		type MaxCredentialTypeLength: Get<u32>;

		/// Origin for privileged actions
		type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
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

	#[pallet::storage]
	#[pallet::getter(fn issued_credentials)]
	pub type IssuedCredentials<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, DidIdentifierOf<T>>, // Subject
			NMapKey<Blake2_128Concat, CredentialOf<T>>,    // CredentialType
			NMapKey<Blake2_128Concat, DidIdentifierOf<T>>, // Issuer
		),
		CredentialInfo<T>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CredentialTypesAdded {
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		},
		CredentialTypesRemoved {
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
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
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
			verifiable_credential_hash: HashOf<T>,
		},
		CredentialsRevoked {
			issuer: DidIdentifierOf<T>,
			did: DidIdentifierOf<T>,
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		},
		CredentialsForcedRevoked {
			issuer: DidIdentifierOf<T>,
			did: DidIdentifierOf<T>,
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		},
		IssuerRemoved {
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
		/// Unable to add credential that already exists
		CredentialTypeAlreadyAdded,
		/// Unable to find credential
		CredentialTypeDoesNotExist,
		/// Unable to find issued credential
		IssuedCredentialDoesNotExist,
		/// Unable to create issuer that already exists
		IssuerAlreadyExists,
		/// Unable to create an issuer that does not have a DID
		IssuerDoesNotHaveDid,
		/// Unable to find issuer
		IssuerDoesNotExist,
		/// Issuer status is not Active
		IssuerNotActive,
		/// Issuer status is not Revoked
		IssuerNotRevoked,
		/// Issuer is deleted and can not be created again
		IssuerIsDeleted,
		/// The origin is not an Issuer
		NotIssuer,
		/// The maximum number of Credentials has been exceeded
		MaxCredentials,
		/// Unable to create DID that already exists
		DidAlreadyExists,
		/// Unable to find DID from DidIdentifier
		DidNotFound,
		/// The origin was not the controller of the DID
		NotController,
		/// Service already exist in the DID document
		ServiceAlreadyInDid,
		/// The service key was not found in the DID
		ServiceNotInDid,
		/// Too many references to a service. Not likely to happen
		TooManyServiceConsumers,
		/// The maximum number of Services in the DID has been exceeded
		TooManyServicesInDid,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_did(services.len() as u32))]
		pub fn create_did(
			origin: OriginFor<T>,
			controller: DidIdentifierOf<T>,
			authentication: T::AuthenticationAddress,
			assertion: Option<T::AssertionAddress>,
			services: BoundedVec<ServiceInfo<T>, T::MaxServices>,
		) -> DispatchResultWithPostInfo {
			let origin = ensure_signed(origin)?;
			let did = T::DidIdentifier::from(origin.clone());

			// Check that DID does not exist yet
			ensure!(!Did::<T>::contains_key(did.clone()), Error::<T>::DidAlreadyExists);

			// Check that we are not re-creating a DID for a Deleted Issuer.
			// If there is a key for an Issuer, and the document does not exist,
			// we can infer that the Issuer had been deleted.
			ensure!(!Issuers::<T>::contains_key(did.clone()), Error::<T>::IssuerIsDeleted);

			// Reserve did deposit.
			// If user does not have enough balance returns `InsufficientBalance`
			T::Currency::reserve(&origin, T::DidDeposit::get())?;

			// Add assertion method
			let maybe_assertion_method =
				assertion.map(|assertion| AssertionMethod::<T> { controller: assertion });

			// For keeping track of Services inserts/removals
			let mut services_witness = ServicesWitness::default();
			// Add services.
			let services_keys = Self::do_add_did_services(
				services,
				&mut <ServiceKeysOf<T>>::default(),
				&mut services_witness,
			)?;

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

			Ok(Some(T::WeightInfo::create_did(services_witness.inserts)).into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(
			T::WeightInfo::update_did().saturating_add(
				T::WeightInfo::add_did_services(services.clone().or_else(|| Some(BoundedVec::default())).expect("value is always Ok").len() as u32)
					.saturating_add(
						T::WeightInfo::remove_did_services(T::MaxServices::get())
					)
			)
		)]
		pub fn update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller: Option<DidIdentifierOf<T>>,
			authentication: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<ServiceInfo<T>, T::MaxServices>>,
		) -> DispatchResultWithPostInfo {
			// For keeping track of Services inserts/removals
			let mut services_witness = ServicesWitness::default();

			let document = Self::do_update_did(
				origin,
				did.clone(),
				controller,
				authentication,
				assertion,
				services.clone(),
				&mut services_witness,
				|origin, document| Self::ensure_controller(ensure_signed(origin)?, document),
			)?;

			Self::deposit_event(Event::DidUpdated { did, document });

			Ok(Some(
				T::WeightInfo::add_did_services(services_witness.inserts)
					.saturating_add(T::WeightInfo::remove_did_services(services_witness.removals)),
			)
			.into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(
			T::WeightInfo::update_did().saturating_add(
				T::WeightInfo::add_did_services(services.clone().or_else(|| Some(BoundedVec::default())).expect("value is always Ok").len() as u32)
					.saturating_add(
						T::WeightInfo::remove_did_services(T::MaxServices::get())
					)
			)
		)]
		pub fn force_update_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			controller: Option<DidIdentifierOf<T>>,
			authentication: Option<T::AuthenticationAddress>,
			assertion: Option<T::AssertionAddress>,
			services: Option<BoundedVec<ServiceInfo<T>, T::MaxServices>>,
		) -> DispatchResultWithPostInfo {
			// For keeping track of Services inserts/removals
			let mut services_witness = ServicesWitness::default();

			let document = Self::do_update_did(
				origin,
				did.clone(),
				controller,
				authentication,
				assertion,
				services,
				&mut services_witness,
				|origin, _| Self::ensure_governance(origin),
			)?;

			Self::deposit_event(Event::DidForcedUpdated { did, document });

			Ok(Pays::No.into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::remove_did(T::MaxServices::get()))]
		pub fn remove_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
		) -> DispatchResultWithPostInfo {
			// For keeping track of Services inserts/removals
			let mut services_witness = ServicesWitness::default();
			Self::do_remove_did(origin, did.clone(), &mut services_witness, |origin, document| {
				Self::ensure_controller(ensure_signed(origin)?, document)
			})?;
			Self::deposit_event(Event::DidRemoved { did });

			Ok(Some(T::WeightInfo::remove_did(services_witness.removals)).into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::remove_did(T::MaxServices::get()))]
		pub fn force_remove_did(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
		) -> DispatchResultWithPostInfo {
			// For keeping track of Services inserts/removals
			let mut services_witness = ServicesWitness::default();
			Self::do_remove_did(origin, did.clone(), &mut services_witness, |origin, _| {
				Self::ensure_governance(origin)
			})?;
			Self::deposit_event(Event::DidForcedRemoved { did });

			Ok(Pays::No.into())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::add_did_services(services.len() as u32))]
		pub fn add_did_services(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			services: BoundedVec<ServiceInfo<T>, T::MaxServices>,
		) -> DispatchResultWithPostInfo {
			let controller = ensure_signed(origin)?;
			// For keeping track of Services inserts/removals
			let mut services_witness = ServicesWitness::default();
			// Try to mutate document
			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> DispatchResultWithPostInfo {
				let document = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;
				Self::ensure_controller(controller, document)?;
				// Insert new services
				let services_keys = Self::do_add_did_services(
					services,
					&mut document.services,
					&mut services_witness,
				)?;

				// if document.services is empty, set it to the new services
				if document.services.is_empty() {
					document.services = services_keys.clone();
				}

				Self::deposit_event(Event::DidServicesAdded { did, new_services: services_keys });

				Ok(Some(T::WeightInfo::add_did_services(services_witness.inserts)).into())
			})
		}

		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::remove_did_services(services_keys.len() as u32))]
		pub fn remove_did_services(
			origin: OriginFor<T>,
			did: DidIdentifierOf<T>,
			services_keys: ServiceKeysOf<T>,
		) -> DispatchResultWithPostInfo {
			let controller = ensure_signed(origin)?;

			// For keeping track of Services inserts/removals
			let mut services_witness = ServicesWitness::default();

			Did::<T>::try_mutate(did.clone(), |maybe_doc| -> DispatchResultWithPostInfo {
				let document = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;
				// ensure that the caller is the controller of the DID
				Self::ensure_controller(controller, document)?;

				Self::do_remove_did_services(
					&services_keys,
					&mut document.services,
					&mut services_witness,
				)?;

				Self::deposit_event(Event::DidServicesRemoved {
					did,
					removed_services: services_keys,
				});

				Ok(Some(T::WeightInfo::remove_did_services(services_witness.removals)).into())
			})
		}

		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::issue_credentials(credentials.len() as u32))]
		pub fn issue_credentials(
			origin: OriginFor<T>,
			issuer_did: DidIdentifierOf<T>,
			subject_did: DidIdentifierOf<T>,
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
			verifiable_credential_hash: HashOf<T>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;

			// Ensure origin is the issuer's controller
			let document = Did::<T>::get(&issuer_did).ok_or(Error::<T>::DidNotFound)?;
			Self::ensure_controller(controller, &document)?;

			// Ensure issuer exists and is active
			Self::ensure_issuer_is_active(&issuer_did)?;

			// Ensure Credential types exist
			Self::ensure_valid_credentials(&credentials)?;

			// Check that subject DID exist
			ensure!(Did::<T>::contains_key(subject_did.clone()), Error::<T>::DidNotFound);

			for credential in credentials.clone() {
				IssuedCredentials::<T>::try_mutate(
					(subject_did.clone(), &credential, issuer_did.clone()),
					|maybe_issued_credential| -> DispatchResult {
						*maybe_issued_credential = Some(CredentialInfo {
							verifiable_credential_hash: verifiable_credential_hash.clone(),
						});
						Ok(())
					},
				)?;
			}

			Self::deposit_event(Event::CredentialsIssued {
				issuer: issuer_did,
				did: subject_did,
				credentials,
				verifiable_credential_hash,
			});
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::revoke_credentials(credentials.len() as u32))]
		pub fn revoke_credentials(
			origin: OriginFor<T>,
			issuer_did: DidIdentifierOf<T>,
			subject_did: DidIdentifierOf<T>,
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;

			// Ensure origin is the issuer's controller
			let document = Did::<T>::get(&issuer_did).ok_or(Error::<T>::DidNotFound)?;
			Self::ensure_controller(controller, &document)?;

			Self::do_revoke_credentials(&issuer_did, &subject_did, &credentials)?;

			Self::deposit_event(Event::CredentialsRevoked {
				issuer: issuer_did,
				did: subject_did,
				credentials,
			});
			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::revoke_credentials(credentials.len() as u32))]
		pub fn force_revoke_credentials(
			origin: OriginFor<T>,
			issuer_did: DidIdentifierOf<T>,
			subject_did: DidIdentifierOf<T>,
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		) -> DispatchResultWithPostInfo {
			T::GovernanceOrigin::ensure_origin(origin.clone())?;

			Self::do_revoke_credentials(&issuer_did, &subject_did, &credentials)?;

			Self::deposit_event(Event::CredentialsForcedRevoked {
				issuer: issuer_did,
				did: subject_did,
				credentials,
			});
			Ok(Pays::No.into())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::add_issuer())]
		pub fn add_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			ensure!(Did::<T>::contains_key(issuer.clone()), Error::<T>::IssuerDoesNotHaveDid);
			ensure!(!Issuers::<T>::contains_key(&issuer), Error::<T>::IssuerAlreadyExists);
			// Add issuer to storage with status Active
			Issuers::<T>::insert(issuer.clone(), IssuerInfo { status: IssuerStatus::Active });
			Self::deposit_event(Event::IssuerStatusActive { issuer });
			Ok(())
		}

		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::revoke_issuer())]
		pub fn revoke_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;

			// Change issuer status to Revoked
			Issuers::<T>::try_mutate(issuer.clone(), |maybe_info| -> DispatchResult {
				let info = maybe_info.as_mut().ok_or(Error::<T>::IssuerDoesNotExist)?;
				Self::ensure_issuer_is_active(&issuer)?;
				*info = IssuerInfo { status: IssuerStatus::Revoked };
				Self::deposit_event(Event::IssuerStatusRevoked { issuer });
				Ok(())
			})
		}

		#[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::reactivate_issuer())]
		pub fn reactivate_issuer(
			origin: OriginFor<T>,
			issuer: DidIdentifierOf<T>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;

			// Change issuer status to Active
			Issuers::<T>::try_mutate(issuer.clone(), |maybe_info| -> DispatchResult {
				let info = maybe_info.as_mut().ok_or(Error::<T>::IssuerDoesNotExist)?;
				ensure!(
					*info != IssuerInfo { status: IssuerStatus::Deleted },
					Error::<T>::IssuerIsDeleted
				);
				ensure!(
					*info == IssuerInfo { status: IssuerStatus::Revoked },
					Error::<T>::IssuerNotRevoked
				);
				*info = IssuerInfo { status: IssuerStatus::Active };
				Self::deposit_event(Event::IssuerStatusReactived { issuer });
				Ok(())
			})
		}

		#[pallet::call_index(13)]
		#[pallet::weight(T::WeightInfo::add_credentials_type(credentials.len() as u32))]
		pub fn add_credentials_type(
			origin: OriginFor<T>,
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			let mut credentials_types = CredentialsTypes::<T>::get();

			for credential in credentials.clone() {
				let pos = credentials_types
					.binary_search(&credential)
					.err()
					.ok_or(Error::<T>::CredentialTypeAlreadyAdded)?;
				credentials_types
					.try_insert(pos, credential.clone())
					.map_err(|_| Error::<T>::MaxCredentials)?;
			}

			CredentialsTypes::<T>::put(credentials_types.clone());
			Self::deposit_event(Event::CredentialTypesAdded { credentials });
			Ok(())
		}

		#[pallet::call_index(14)]
		#[pallet::weight(T::WeightInfo::remove_credentials_type(credentials.len() as u32))]
		pub fn remove_credentials_type(
			origin: OriginFor<T>,
			credentials: BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			let mut credentials_types = CredentialsTypes::<T>::get();

			for credential in credentials.clone() {
				let pos = credentials_types
					.binary_search(&credential)
					.ok()
					.ok_or(Error::<T>::CredentialTypeDoesNotExist)?;
				credentials_types.remove(pos);
			}

			CredentialsTypes::<T>::put(credentials_types.clone());
			Self::deposit_event(Event::CredentialTypesRemoved { credentials });
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Updates `document` with specified fields. Inserting services may fail.
	fn do_update_did(
		origin: OriginFor<T>,
		did: DidIdentifierOf<T>,
		controller: Option<DidIdentifierOf<T>>,
		authentication: Option<T::AuthenticationAddress>,
		assertion: Option<T::AssertionAddress>,
		services: Option<BoundedVec<ServiceInfo<T>, T::MaxServices>>,
		services_witness: &mut ServicesWitness,
		origin_check: impl FnOnce(OriginFor<T>, &Document<T>) -> DispatchResult,
	) -> Result<Document<T>, DispatchError> {
		Did::<T>::try_mutate(did, |maybe_doc| -> Result<Document<T>, DispatchError> {
			let document = maybe_doc.as_mut().ok_or(Error::<T>::DidNotFound)?;

			// Check if origin is either governance or controller
			origin_check(origin, document)?;

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
				// Clean all original services
				Self::do_remove_did_services(
					&document.services,
					&mut document.services.clone(),
					services_witness,
				)?;
				// Add all the new services
				document.services = Self::do_add_did_services(
					new_services,
					&mut <ServiceKeysOf<T>>::default(),
					services_witness,
				)?;
			}

			Ok(document.clone())
		})
	}

	fn do_remove_did(
		origin: OriginFor<T>,
		did: DidIdentifierOf<T>,
		services_witness: &mut ServicesWitness,
		origin_check: impl FnOnce(OriginFor<T>, &Document<T>) -> DispatchResult,
	) -> DispatchResult {
		Did::<T>::try_mutate(did.clone(), |maybe_doc| -> DispatchResult {
			// Take from storage (sets to None). Will be deleted if successful
			let document = maybe_doc.take().ok_or(Error::<T>::DidNotFound)?;

			// Check if origin is either governance or controller
			origin_check(origin, &document)?;

			Self::do_remove_did_services(
				&document.services,
				&mut document.services.clone(),
				services_witness,
			)?;

			// If DID belongs to Issuer, attempt to remove it
			if Issuers::<T>::contains_key(did.clone()) {
				Self::do_remove_issuer(did.clone())?;
			}

			T::Currency::unreserve(&did.clone().into(), T::DidDeposit::get());
			Ok(())
		})
	}

	fn do_add_did_services(
		services_to_add: BoundedVec<ServiceInfo<T>, T::MaxServices>,
		document_services_keys: &mut ServiceKeysOf<T>,
		services_witness: &mut ServicesWitness,
	) -> Result<ServiceKeysOf<T>, DispatchError> {
		let mut services_keys = <ServiceKeysOf<T>>::default();

		for service in services_to_add {
			let service_key = Self::do_add_service(service, services_witness)?;
			if !document_services_keys.is_empty() {
				let pos = document_services_keys
					.binary_search(&service_key)
					.err()
					.ok_or(Error::<T>::ServiceAlreadyInDid)?;
				document_services_keys
					.try_insert(pos, service_key)
					.map_err(|_| Error::<T>::TooManyServicesInDid)?;
			}

			let pos = services_keys
				.binary_search(&service_key)
				.err()
				.ok_or(Error::<T>::ServiceAlreadyInDid)?;
			services_keys
				.try_insert(pos, service_key)
				.map_err(|_| Error::<T>::TooManyServicesInDid)?;
		}
		Ok(services_keys)
	}

	/// Insert a single service into storage
	fn do_add_service(
		service: ServiceInfo<T>,
		services_witness: &mut ServicesWitness,
	) -> Result<KeyIdOf<T>, DispatchError> {
		let service_key = T::Hashing::hash_of(&service);

		// if the service exists increment its consumers, otherwise insert a new service
		if let Some(mut existing_service) = Services::<T>::get(service_key) {
			// `inc_consumers` may overflow, so handle it just in case
			existing_service
				.inc_consumers()
				.map_err(|_| Error::<T>::TooManyServiceConsumers)?;
			<Services<T>>::set(service_key, Some(existing_service));
		} else {
			<Services<T>>::insert(service_key, Service::<T>::new(service));
			services_witness.inserts =
				services_witness.inserts.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		}

		Ok(service_key)
	}

	fn do_remove_did_services(
		keys_to_remove: &ServiceKeysOf<T>,
		document_services_keys: &mut ServiceKeysOf<T>,
		services_witness: &mut ServicesWitness,
	) -> DispatchResult {
		// Iterate over each service and remove it from the document
		for service_key in keys_to_remove {
			let pos = document_services_keys
				.binary_search(service_key)
				.ok()
				.ok_or(Error::<T>::ServiceNotInDid)?;
			let deleted_key = document_services_keys.remove(pos);

			// House cleaning. Check consumers and possibly delete from storage
			Self::do_remove_service(deleted_key, services_witness)?;
		}
		Ok(())
	}

	// Decrements consumers and removes from storage if consumers == 0
	fn do_remove_service(
		service_key: KeyIdOf<T>,
		services_witness: &mut ServicesWitness,
	) -> DispatchResult {
		Services::<T>::try_mutate_exists(
			service_key,
			|maybe_service| -> Result<(), DispatchError> {
				let service = maybe_service.as_mut().ok_or(Error::<T>::ServiceNotInDid)?;

				service.dec_consumers();
				// Delete from storage if consumers == 0
				if service.consumers() == 0 {
					*maybe_service = None;
					services_witness.removals = services_witness
						.removals
						.checked_add(1)
						.ok_or(ArithmeticError::Overflow)?;
				}

				Ok(())
			},
		)
	}

	fn do_remove_issuer(issuer: DidIdentifierOf<T>) -> DispatchResult {
		Issuers::<T>::try_mutate_exists(issuer.clone(), |maybe_info| -> DispatchResult {
			// Get mutable reference to issuer info. Will not be removed from storage
			let info = maybe_info.as_mut().ok_or(Error::<T>::IssuerDoesNotExist)?;
			ensure!(
				*info == IssuerInfo { status: IssuerStatus::Revoked },
				Error::<T>::IssuerNotRevoked
			);

			// Update status to `Deleted` to prevent issuer from being reinstated
			info.status = IssuerStatus::Deleted;

			Self::deposit_event(Event::IssuerRemoved { issuer });
			Ok(())
		})
	}

	fn do_revoke_credentials(
		issuer_did: &DidIdentifierOf<T>,
		subject_did: &DidIdentifierOf<T>,
		credentials: &Vec<CredentialOf<T>>,
	) -> DispatchResult {
		for credential in credentials {
			IssuedCredentials::<T>::try_mutate_exists(
				(subject_did, &credential, issuer_did),
				|maybe_issued_credential| -> DispatchResult {
					maybe_issued_credential
						.take()
						.ok_or(Error::<T>::IssuedCredentialDoesNotExist)?;
					Ok(())
				},
			)?;
		}
		Ok(())
	}

	/// Ensures that `who` is the controller of the did document
	fn ensure_controller(who: T::AccountId, document: &Document<T>) -> DispatchResult {
		ensure!(document.controller == T::DidIdentifier::from(who), Error::<T>::NotController);
		Ok(())
	}

	/// Ensures that origin is governance
	fn ensure_governance(origin: OriginFor<T>) -> DispatchResult {
		T::GovernanceOrigin::ensure_origin(origin)?;
		Ok(())
	}

	/// Ensure that the issuer status is active
	fn ensure_issuer_is_active(issuer_did: &DidIdentifierOf<T>) -> DispatchResult {
		let issuer_info = Issuers::<T>::get(issuer_did).ok_or(Error::<T>::NotIssuer)?;
		ensure!(issuer_info.status == IssuerStatus::Active, Error::<T>::IssuerNotActive);
		Ok(())
	}

	/// Ensure that the credential type exists
	fn ensure_valid_credentials(credentials: &Vec<CredentialOf<T>>) -> DispatchResult {
		let credential_types = <CredentialsTypes<T>>::get();

		for credential in credentials {
			credential_types
				.binary_search(credential)
				.ok()
				.ok_or(Error::<T>::CredentialTypeDoesNotExist)?;
		}
		Ok(())
	}
}
