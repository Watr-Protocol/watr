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

use super::*;
use crate as pallet_did;
use crate::{mock::*, Event as MotionEvent};
use codec::Encode;
use frame_support::{assert_ok, dispatch::GetDispatchInfo, weights::Weight};
use frame_system::{EventRecord, Phase};
use mock::{RuntimeCall, RuntimeEvent};
use sp_core::{H160, H256};
use sp_runtime::{bounded_vec, traits::{BlakeTwo256, Hash}};

fn record(event: RuntimeEvent) -> EventRecord<RuntimeEvent, H256> {
	EventRecord { phase: Phase::Initialization, event, topics: vec![] }
}

fn events() -> Vec<Event<Test>> {
	let result = System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let mock::RuntimeEvent::DID(inner) = e { Some(inner) } else { None })
		.collect::<Vec<_>>();

	System::reset_events();

	result
}

fn default_services() -> BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices> {
	bounded_vec![
		ServiceInfo {
			type_id: types::ServiceType::VerifiableCredentialFileStorage,
			service_endpoint: bounded_vec![b's', b'0']
		},
		ServiceInfo {
			type_id: types::ServiceType::VerifiableCredentialFileStorage,
			service_endpoint: bounded_vec![b's', b'1']
		}
	]
}

fn create_default_did(origin_id: u64) -> Document<Test> {
	let origin = RuntimeOrigin::signed(origin_id);
	let controller = 1;
	let authentication: H160 = H160::from([0u8; 20]);
	let assertion: H160 = H160::from([0u8; 20]);
	let services = default_services();
	let mut services_keys = hash_services(&services);
	services_keys.sort();

	let expected_document = Document {
		controller,
		authentication: AuthenticationMethod { controller: authentication },
		assertion_method: Some ( AssertionMethod { controller: assertion } ),
		services: services_keys
	};

	assert_ok!(DID::create_did(origin, controller, authentication, Some(assertion), services));
	expected_document
}

fn hash_services(services: &BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices>) -> ServiceKeysOf<Test> {
	let mut services_keys: ServiceKeysOf<Test> = BoundedVec::default();
	for service in services {
		services_keys.try_push(<mock::Test as frame_system::Config>::Hashing::hash_of(&service));
	}
	// services_keys.sort();
	services_keys
}

fn assert_services(services_info: BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices>, expected_consumers: u32) {
	let services_keys = hash_services(&services_info);

	for (i, key) in services_keys.iter().enumerate() {
		let service = DID::services(key).unwrap();
		assert_eq!(service.consumers(), expected_consumers);
		assert_eq!(service.info, services_info[i]);
	}
}

fn assert_services_do_not_exist(services_info: BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices>) {
	let services_keys = hash_services(&services_info);

	for key in services_keys {
		assert_eq!(DID::services(key), None);
	}
}

#[test]
fn create_did_works() {
	new_test_ext().execute_with(|| {
		// inserts default DID into storage. Checks for Ok()
		let expected_document = create_default_did(ALICE);

		assert_eq!(Balances::reserved_balance(&ALICE), DidDeposit::get());
		assert_eq!(DID::dids(ALICE), Some(expected_document.clone()));
		assert_services(default_services(), 1);
		assert!(events().contains(&Event::<Test>::DidCreated {
			did: ALICE,
			document: expected_document,
		}));
	});
}

#[test]
fn update_did_works() {
	new_test_ext().execute_with(|| {
		let mut old_document = create_default_did(ALICE);

		let origin = RuntimeOrigin::signed(ALICE);
		let controller = 2;
		let authentication: H160 = H160::from([1u8; 20]);
		let assertion: H160 = H160::from([1u8; 20]);
		let mut services = default_services();
		services[0].service_endpoint = bounded_vec![b's', b'2'];
		services[1].service_endpoint = bounded_vec![b's', b'3'];

		let mut services_keys = hash_services(&services);
		services_keys.sort();

		let expected_document = Document {
			controller,
			authentication: AuthenticationMethod { controller: authentication },
			assertion_method: Some ( AssertionMethod { controller: assertion } ),
			services: services_keys
		};


		assert_ok!(DID::update_did(
			origin,
			ALICE,
			Some(controller),
			Some(authentication),
			Some(assertion),
			Some(services.clone())
		));
		assert_eq!(DID::dids(ALICE), Some(expected_document.clone()));
		// assert services exist and have a consumer count of 1
		assert_services(services, 1);
		// assert that the default services were removed from storage
		assert_services_do_not_exist(default_services());
		assert!(events().contains(&Event::<Test>::DidUpdated {
			did: ALICE,
			document: expected_document,
		}));

	});
}

#[test]
fn force_update_did_works() {
	new_test_ext().execute_with(|| {
		// default did with origin & did for Alice
		let mut old_document = create_default_did(ALICE);

		let origin = RuntimeOrigin::root();
		let controller = 2;
		let authentication: H160 = H160::from([1u8; 20]);
		let assertion: H160 = H160::from([1u8; 20]);
		let mut services = default_services();
		services[0].service_endpoint = bounded_vec![b's', b'2'];
		services[1].service_endpoint = bounded_vec![b's', b'3'];

		let services_keys = hash_services(&services);

		let expected_document = Document {
			controller,
			authentication: AuthenticationMethod { controller: authentication },
			assertion_method: Some ( AssertionMethod { controller: assertion } ),
			services: services_keys
		};


		assert_ok!(DID::force_update_did(
			origin,
			ALICE,
			Some(controller),
			Some(authentication),
			Some(assertion),
			Some(services.clone())
		));
		assert_eq!(DID::dids(ALICE), Some(expected_document.clone()));
		// assert services exist and have a consumer count of 1
		assert_services(services, 1);
		// assert that the default services were removed from storage
		assert_services_do_not_exist(default_services());
		assert!(events().contains(&Event::<Test>::DidForcedUpdated {
			did: ALICE,
			document: expected_document,
		}));
	});
}

#[test]
fn remove_did_works() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		// inserts default DID into storage. Checks for Ok()
		let expected_document = create_default_did(ALICE);

		assert_ok!(DID::remove_did(origin, ALICE));
		assert_eq!(DID::dids(ALICE), None);
		assert_eq!(Balances::reserved_balance(&ALICE), 0);
		assert_services_do_not_exist(default_services());
		assert!(events().contains(&Event::<Test>::DidRemoved {
			did: ALICE,
		}));
	});
}

#[test]
fn force_remove_did_works() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::root();
		// inserts default DID into storage. Checks for Ok()
		let expected_document = create_default_did(ALICE);

		assert_ok!(DID::force_remove_did(origin, ALICE));
		assert_eq!(DID::dids(ALICE), None);
		assert_eq!(Balances::reserved_balance(&ALICE), 0);
		assert_services_do_not_exist(default_services());
		assert!(events().contains(&Event::<Test>::DidForcedRemoved {
			did: ALICE,
		}));
	});
}

#[test]
fn add_did_services_works() {
	new_test_ext().execute_with(|| {
		let mut old_document = create_default_did(ALICE);

		let origin = RuntimeOrigin::signed(ALICE);

		let mut new_services = default_services();
		new_services[0].service_endpoint = bounded_vec![b's', b'2'];
		new_services[1].service_endpoint = bounded_vec![b's', b'3'];

		let new_services_keys = hash_services(&new_services);
		let mut combined_services = old_document.services.clone();
		new_services_keys.clone().into_iter().for_each(|key| {
			combined_services.try_push(key);
		});
		combined_services.sort();

		let expected_document = Document {
			services: combined_services,
			..old_document
		};

		assert_ok!(DID::add_did_services(
			origin,
			ALICE,
			new_services.clone()
		));
		assert_eq!(DID::dids(ALICE), Some(expected_document.clone()));
		// assert services exist and have a consumer count of 1
		assert_services(new_services.clone(), 1);
		// assert that the default services were removed from storage
		// assert_services_do_not_exist(default_services());
		assert!(events().contains(&Event::<Test>::DidServicesAdded {
			did: ALICE,
			new_services: new_services_keys,
		}));

	});
}