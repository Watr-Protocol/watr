use crate::*;
use frame_benchmarking::{account, benchmarks, whitelist_account, whitelisted_caller};
use frame_support::{assert_ok, traits::OriginTrait};
use frame_system::RawOrigin;
use sp_core::H160;
use sp_runtime::traits::Bounded;

use super::{Pallet as DID, *};

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

fn create_service<T: Config>(i: u32) -> ServiceInfo<T> {
	let mut service_endpoint = BoundedVec::default();
	let service = i.to_be_bytes();

	for b in service {
		service_endpoint.try_push(b);
	}

	ServiceInfo { type_id: ServiceType::VerifiableCredentialFileStorage, service_endpoint }
}

fn create_services<T: Config>(
	m: u32,
) -> (BoundedVec<ServiceInfo<T>, T::MaxServices>, BoundedVec<KeyIdOf<T>, T::MaxServices>) {
	let mut services: BoundedVec<ServiceInfo<T>, T::MaxServices> = BoundedVec::default();
	for i in 0..m {
		let service = create_service::<T>(i);
		services.try_push(service);
	}

	let mut services_keys: BoundedVec<KeyIdOf<T>, T::MaxServices> = BoundedVec::default();
	for service in &services {
		let key = T::Hashing::hash_of(&service);
		let pos = services_keys.binary_search(&key).err().unwrap();
		services_keys.try_insert(pos, key.clone());
	}
	(services, services_keys)
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

		let (services, services_keys) = create_services::<T>(m);
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
		let n in 0 .. T::MaxServices::get(); // Existing services with consumer = 1 to be removed
		let m in 0 .. T::MaxServices::get(); // New services to be added

		// Dependancy - Create a DID with its document
		let mut controller_id = 1;
		let mut authentication_id = 1;
		let mut assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(n);
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
		let (new_services, new_services_keys) = create_services::<T>(m);
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

	// ---------------------------------------------
	remove_did {
		let n in 0 .. T::MaxServices::get(); // Existing services with consumers = 1 to be removed

		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(n);
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

	// ---------------------------------------------
	add_did_services {
		let n = 0; // No existing services
		let m in 0 .. T::MaxServices::get(); // New services to be added

		// Dependancy - Create a DID with its document
		let mut controller_id = 1;
		let mut authentication_id = 1;
		let mut assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(n);
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
		let (new_services, new_services_keys) = create_services::<T>(m);
		let new_document: Document<T> = create_did_document(controller_id, authentication_id, assertion_id, &new_services_keys);

	}: _(controller_origin, T::DidIdentifier::from(did.clone()), new_services)
	verify {
		assert_eq!(Did::get(T::DidIdentifier::from(did.clone())), Some(new_document));
		assert_last_event::<T>(Event::DidServicesAdded { did: T::DidIdentifier::from(did), new_services: new_services_keys.clone() }.into());
	}

	// ---------------------------------------------
	remove_did_services {
		let n in 0 .. T::MaxServices::get(); // Existing services with consumers = 1 to be removed
		let m in 0 .. T::MaxServices::get(); // Services to be removed

		// Dependancy - Create a DID with its document
		let controller_id = 1;
		let authentication_id = 1;
		let assertion_id = 1;
		let (existing_services, existing_services_keys) = create_services::<T>(n);
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
		let mut i = if m <= n { m } else { 0 };
		let (services_to_remove, services_keys_to_remove) = create_services::<T>(i);
	}: _(controller_origin, T::DidIdentifier::from(did.clone()), services_keys_to_remove.clone())
	verify {
		if m <= n {
			assert_last_event::<T>(Event::DidServicesRemoved { did: T::DidIdentifier::from(did), removed_services: services_keys_to_remove }.into());
		}
	}

	// // ---------------------------------------------
	// issue_credentials {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// revoke_credentials {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// add_issuer {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// revoke_issuer {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// reactivate_issuer {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// remove_issuer {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// add_credentials_type {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// remove_credentials_type {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }
}

// impl_benchmark_test_suite!(
// 	MyPallet,
// 	crate::mock::new_test_ext(),
// 	crate::mock::Test,
// );
