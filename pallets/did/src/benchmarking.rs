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

use crate::*;
use frame_benchmarking::{account, benchmarks, whitelist_account, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_core::H160;
use sp_runtime::traits::Bounded;

use super::{types::ServiceType, Pallet as DID};

const SEED: u32 = 0;

fn controller<T: Config>(i: u32) -> DidIdentifierOf<T> {
	let account = account::<T::AccountId>("controller", i, SEED);
	whitelist_account!(account);
	T::DidIdentifier::from(account)
}

fn issuer<T: Config>(i: u32) -> DidIdentifierOf<T> {
	let account = account::<T::AccountId>("issuer", i, SEED);
	whitelist_account!(account);
	T::DidIdentifier::from(account)
}

fn authentication<T: Config>(i: u64) -> T::AuthenticationAddress {
	H160::from_low_u64_be(i).into()
}

fn assertion<T: Config>(i: u64) -> T::AssertionAddress {
	H160::from_low_u64_be(i).into()
}

fn create_service<T: Config>(i: u32, seed: u8) -> ServiceInfo<T> {
	let mut service_endpoint = BoundedVec::default();
	let service = i.to_be_bytes();

	for b in service {
		let _ = service_endpoint.try_push(b);
	}

	let _ = service_endpoint.try_push(seed);

	ServiceInfo { type_id: ServiceType::VerifiableCredentialFileStorage, service_endpoint }
}

fn create_services<T: Config>(
	i: u32,
	seed: u8,
) -> (BoundedVec<ServiceInfo<T>, T::MaxServices>, BoundedVec<KeyIdOf<T>, T::MaxServices>) {
	let mut services: BoundedVec<ServiceInfo<T>, T::MaxServices> = BoundedVec::default();
	for j in 0..i {
		let service = create_service::<T>(j, seed);
		let _ = services.try_push(service);
	}

	let mut services_keys: BoundedVec<KeyIdOf<T>, T::MaxServices> = BoundedVec::default();
	for service in &services {
		let key = T::Hashing::hash_of(&service);
		let pos = services_keys.binary_search(&key).err().unwrap();
		let _ = services_keys.try_insert(pos, key.clone());
	}
	(services, services_keys)
}

fn create_credential<T: Config>(i: u32, seed: u8) -> CredentialOf<T> {
	let mut credential_type = CredentialOf::<T>::default();
	let credential = i.to_be_bytes();

	for b in credential {
		let _ = credential_type.try_push(b);
	}

	let _ = credential_type.try_push(seed);
	credential_type
}

fn create_credentials<T: Config>(
	i: u32,
	seed: u8,
) -> BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes> {
	let mut credentials = BoundedVec::<CredentialOf<T>, T::MaxCredentialsTypes>::default();
	for j in 0..i {
		let credential = create_credential::<T>(j, seed);
		let _ = credentials.try_push(credential);
	}
	credentials
}

fn create_did_document<T: Config>(
	controller_id: u32,
	authentication_id: u64,
	assertion_id: u64,
	services_keys: &BoundedVec<KeyIdOf<T>, T::MaxServices>,
) -> Document<T> {
	let controller = controller::<T>(controller_id);
	let authentication = authentication::<T>(authentication_id);
	let assertion = Some(assertion::<T>(assertion_id));

	Document {
		controller: controller.clone(),
		authentication: AuthenticationMethod::<T> { controller: authentication.clone() },
		assertion_method: Some(AssertionMethod::<T> { controller: assertion.clone().unwrap() }),
		services: services_keys.clone(),
	}
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	create_did {
		let m in 0 .. T::MaxServices::get(); // New services to be added

		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let services_generator_seed = 1;

		let (services, services_keys) = create_services::<T>(m, services_generator_seed);
		let document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &services_keys);

		let did: T::AccountId = whitelisted_caller();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());
	}: _(
			did_origin,
			document.clone().controller,
			document.clone().authentication.controller,
			Some(document.clone().assertion_method.unwrap().controller),
			services
		)
	verify {
		assert_eq!(Did::get(T::DidIdentifier::from(did.clone())), Some(document.clone()));
		assert_last_event::<T>(Event::DidCreated { did: T::DidIdentifier::from(did), document }.into());
	}

	update_did {
		// update_did purposely does not add or remove services. These are accounted for with
		// add_did_services and remove_did_services

		// Dependancy - Create a DID with its document
		let mut controller_id = 1;
		let mut authentication_id = 1;
		let mut assertion_id = 1;
		let mut services_generator_seed = 1;

		let (existing_services, existing_services_keys) = create_services::<T>(0, services_generator_seed);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did: T::AccountId = whitelisted_caller();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());

		assert!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		).is_ok());

		let controller = controller::<T>(controller_id).into();
		let controller_origin = RawOrigin::Signed(controller.clone());
		T::Currency::make_free_balance_be(&controller, BalanceOf::<T>::max_value());

		// Update DID document with new values
		controller_id = 2;
		authentication_id = 2;
		assertion_id = 2;
		services_generator_seed = 2;

		let (new_services, new_services_keys) = create_services::<T>(0, services_generator_seed);
		let new_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &new_services_keys);
	}: _(
			controller_origin,
			T::DidIdentifier::from(did.clone()),
			Some(new_document.clone().controller),
			Some(new_document.clone().authentication.controller),
			Some(new_document.clone().assertion_method.unwrap().controller),
			Some(new_services)
		)
	verify {
		assert_eq!(Did::get(T::DidIdentifier::from(did.clone())), Some(new_document.clone()));
		assert_last_event::<T>(Event::DidUpdated { did: T::DidIdentifier::from(did), document: new_document }.into());
	}

	remove_did {
		let m in 0 .. T::MaxServices::get(); // Existing services with consumers = 1 to be removed

		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let services_generator_seed = 1;

		let (existing_services, existing_services_keys) = create_services::<T>(m, services_generator_seed);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did: T::AccountId = whitelisted_caller();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());

		assert!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		).is_ok());

		let controller = controller::<T>(controller_id).into();
		let controller_origin = RawOrigin::Signed(controller.clone());
		T::Currency::make_free_balance_be(&controller, BalanceOf::<T>::max_value());
	}: _(controller_origin, T::DidIdentifier::from(did.clone()))
	verify {
		let none: Option<Document<T>> = None;
		assert_eq!(Did::get(T::DidIdentifier::from(did.clone())), none);
		assert_last_event::<T>(Event::DidRemoved { did: T::DidIdentifier::from(did) }.into());
	}

	add_did_services {
		let m in 0 .. T::MaxServices::get(); // New services to be added

		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let services_generator_seed = 1;

		let (existing_services, existing_services_keys) = create_services::<T>(0, services_generator_seed);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did: T::AccountId = whitelisted_caller();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());

		assert!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		).is_ok());

		let controller = controller::<T>(controller_id).into();
		let controller_origin = RawOrigin::Signed(controller.clone());
		T::Currency::make_free_balance_be(&controller, BalanceOf::<T>::max_value());

		// Generate new services to be added
		let (new_services, new_services_keys) = create_services::<T>(m, services_generator_seed);
		let new_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &new_services_keys);

	}: _(controller_origin, T::DidIdentifier::from(did.clone()), new_services)
	verify {
		assert_eq!(Did::get(T::DidIdentifier::from(did.clone())), Some(new_document));
		assert_last_event::<T>(Event::DidServicesAdded { did: T::DidIdentifier::from(did), new_services: new_services_keys.clone() }.into());
	}

	remove_did_services {
		let m in 0 .. T::MaxServices::get(); // Services to be removed

		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let services_generator_seed = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(m, services_generator_seed);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did: T::AccountId = whitelisted_caller();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());

		assert_ok!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		));

		let controller = controller::<T>(controller_id).into();
		let controller_origin = RawOrigin::Signed(controller.clone());
		T::Currency::make_free_balance_be(&controller, BalanceOf::<T>::max_value());

		// Generate services to be removed
		let (services_to_remove, services_keys_to_remove) = create_services::<T>(m, services_generator_seed);

		let new_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &BoundedVec::default());

	}: _(controller_origin, T::DidIdentifier::from(did.clone()), services_keys_to_remove.clone())
	verify {
		assert_eq!(Did::get(T::DidIdentifier::from(did.clone())), Some(new_document));
		assert_last_event::<T>(Event::DidServicesRemoved { did: T::DidIdentifier::from(did), removed_services: services_keys_to_remove }.into());
	}

	issue_credentials {
		let c in 0 .. T::MaxCredentialsTypes::get(); // New credentials to be issued

		let root: T::RuntimeOrigin = RawOrigin::Root.into();

		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(0, 1);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did: T::AccountId = whitelisted_caller();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());

		assert_ok!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		));

		let controller_id = 2;
		let authentication_id = 2;
		let assertion_id = 2;
		let (existing_services, existing_services_keys) = create_services::<T>(0, 1);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);

		let issuer_did = issuer::<T>(2).into();
		let issuer_origin = RawOrigin::Signed(issuer_did.clone());
		T::Currency::make_free_balance_be(&issuer_did, BalanceOf::<T>::max_value());

		let controller = controller::<T>(controller_id).into();
		let controller_origin = RawOrigin::Signed(controller.clone());
		T::Currency::make_free_balance_be(&controller, BalanceOf::<T>::max_value());

		assert_ok!(DID::create_did(
			issuer_origin.clone().into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		));
		assert_ok!(DID::<T>::add_issuer(root.clone(), T::DidIdentifier::from(issuer_did.clone())));

		let credentials = create_credentials::<T>(c, 1);

		let mut verifiable_credential_hash: HashOf<T> = HashOf::<T>::default();
		for i in 0..T::MaxHash::get() {
			let _ = verifiable_credential_hash.try_push((i + c) as u8);
		}

		assert_ok!(DID::<T>::add_credentials_type(root.clone(), credentials.clone()));

	}: _(controller_origin.clone(), T::DidIdentifier::from(issuer_did.clone()), T::DidIdentifier::from(did.clone()), credentials.clone(), verifiable_credential_hash.clone())
	verify {
		for credential in &credentials {
			assert_eq!(IssuedCredentials::<T>::get((T::DidIdentifier::from(did.clone()), credential, T::DidIdentifier::from(issuer_did.clone()))), Some(CredentialInfo {
				verifiable_credential_hash: verifiable_credential_hash.clone()
			}));
		}

		assert_last_event::<T>(Event::CredentialsIssued {
			issuer: T::DidIdentifier::from(issuer_did.clone()),
			did: T::DidIdentifier::from(did.clone()),
			credentials: credentials.clone(),
			verifiable_credential_hash
		}.into());
	}

	revoke_credentials {
		let c in 0 .. T::MaxCredentialsTypes::get(); // New credentials to be issued

		let root: T::RuntimeOrigin = RawOrigin::Root.into();

		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(0, 1);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did: T::AccountId = whitelisted_caller();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());

		assert_ok!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		));
		let controller_id = 2;
		let authentication_id = 2;
		let assertion_id = 2;
		let (existing_services, existing_services_keys) = create_services::<T>(0, 1);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let issuer_did = issuer::<T>(2).into();
		let issuer_origin = RawOrigin::Signed(issuer_did.clone());
		T::Currency::make_free_balance_be(&issuer_did, BalanceOf::<T>::max_value());

		let controller = controller::<T>(controller_id).into();
		let controller_origin = RawOrigin::Signed(controller.clone());
		T::Currency::make_free_balance_be(&controller, BalanceOf::<T>::max_value());

		assert_ok!(DID::create_did(
			issuer_origin.clone().into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		));
		assert_ok!(DID::<T>::add_issuer(root.clone(), T::DidIdentifier::from(issuer_did.clone())));

		let credentials = create_credentials::<T>(c, 1);

		let mut verifiable_credential_hash: HashOf<T> = HashOf::<T>::default();
		for i in 0..T::MaxHash::get() {
			let _ = verifiable_credential_hash.try_push((i + c) as u8);
		}

		assert_ok!(DID::<T>::add_credentials_type(root.clone(), credentials.clone()));
		assert_ok!(DID::<T>::issue_credentials(
			controller_origin.clone().into(),
			T::DidIdentifier::from(issuer_did.clone()),
			T::DidIdentifier::from(did.clone()),
			credentials.clone(),
			verifiable_credential_hash.clone()
		));
	}: _(controller_origin.clone(), T::DidIdentifier::from(issuer_did.clone()), T::DidIdentifier::from(did.clone()), credentials.clone())
	verify {
		for credential in &credentials {
			assert_eq!(IssuedCredentials::<T>::get((T::DidIdentifier::from(did.clone()), credential, T::DidIdentifier::from(issuer_did.clone()))), None);
		}
		assert_last_event::<T>(Event::CredentialsRevoked {
			issuer: T::DidIdentifier::from(issuer_did.clone()),
			did: T::DidIdentifier::from(did.clone()),
			credentials: credentials.clone(),
		}.into());
	}

	add_issuer {
		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(0, 1);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did = controller::<T>(controller_id).into();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());
		assert!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		).is_ok());

		let issuer = T::DidIdentifier::from(did);
		let info = IssuerInfo { status: IssuerStatus::Active };
	}: _(
		RawOrigin::Root, issuer.clone()
	)
	verify {
		assert_eq!(Issuers::<T>::get(issuer.clone()), Some(info));
		assert_last_event::<T>(Event::IssuerStatusActive {issuer: issuer}.into());
	}

	revoke_issuer {
		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(0, 1);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did = controller::<T>(controller_id).into();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());
		assert!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		).is_ok());

		let issuer = T::DidIdentifier::from(did);
		let info = IssuerInfo { status: IssuerStatus::Revoked };
		assert_ok!(DID::<T>::add_issuer(RawOrigin::Root.into(), issuer.clone()));
	}: _(
		RawOrigin::Root, issuer.clone()
	)
	verify {
		assert_eq!(Issuers::<T>::get(issuer.clone()), Some(info));
		assert_last_event::<T>(Event::IssuerStatusRevoked {issuer: issuer}.into());
	}

	reactivate_issuer {
		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(0, 1);
		let existing_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &existing_services_keys);
		let did = controller::<T>(controller_id).into();
		let did_origin = RawOrigin::Signed(did.clone());
		T::Currency::make_free_balance_be(&did, BalanceOf::<T>::max_value());
		assert!(DID::create_did(
			did_origin.into(),
			existing_document.clone().controller,
			existing_document.clone().authentication.controller,
			Some(existing_document.clone().assertion_method.unwrap().controller),
			existing_services
		).is_ok());

		let issuer = T::DidIdentifier::from(did);
		let info = IssuerInfo { status: IssuerStatus::Active };
		assert_ok!(DID::<T>::add_issuer(RawOrigin::Root.into(), issuer.clone()));
		assert_ok!(DID::<T>::revoke_issuer(RawOrigin::Root.into(), issuer.clone()));
	}: _(
		RawOrigin::Root, issuer.clone()
	)
	verify {
		assert_eq!(Issuers::<T>::get(issuer.clone()), Some(info));
		assert_last_event::<T>(Event::IssuerStatusReactived {issuer: issuer}.into());
	}

	add_credentials_type {
		let m in 0 .. T::MaxCredentialsTypes::get();
		let credentials_types = create_credentials::<T>(m, 1);
	}: _(
		RawOrigin::Root, credentials_types.clone()
	)
	verify {
		assert_eq!(CredentialsTypes::<T>::get(), credentials_types);
		assert_last_event::<T>(Event::CredentialTypesAdded {credentials: credentials_types}.into());
	}

	remove_credentials_type {
		let m in 0 .. T::MaxCredentialsTypes::get();
		let credentials_types = create_credentials::<T>(m, 1);

		assert_ok!(DID::<T>::add_credentials_type(RawOrigin::Root.into(), credentials_types.clone()));
	}: _(
		RawOrigin::Root, credentials_types.clone()
	)
	verify {
		assert_eq!(CredentialsTypes::<T>::get(), Vec::default());
		assert_last_event::<T>(Event::CredentialTypesRemoved {credentials: credentials_types}.into());
	}
}
