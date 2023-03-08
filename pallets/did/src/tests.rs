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
use frame_support::{
	assert_noop, assert_ok, bounded_vec, dispatch::GetDispatchInfo, error::BadOrigin,
	weights::Weight,
};
use frame_system::{EventRecord, Phase};
use mock::{RuntimeCall, RuntimeEvent};
use sp_core::{ConstU8, H160, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};

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

fn default_services() -> BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices>
{
	bounded_vec![
		ServiceInfo {
			type_id: types::ServiceType::VerifiableCredentialFileStorage,
			service_endpoint: bounded_vec![b's', b'0']
		},
		ServiceInfo {
			type_id: types::ServiceType::VerifiableCredentialFileStorage,
			service_endpoint: bounded_vec![b's', b'1']
		},
		ServiceInfo {
			type_id: types::ServiceType::VerifiableCredentialFileStorage,
			service_endpoint: bounded_vec![b's', b'2']
		}
	]
}

fn create_default_did(origin_id: u64, controller: u64) -> Document<Test> {
	let origin = RuntimeOrigin::signed(origin_id);
	let controller = controller;
	let authentication: H160 = H160::from([0u8; 20]);
	let assertion: H160 = H160::from([0u8; 20]);
	let services = default_services();
	let mut services_keys = hash_services(&services);
	services_keys.sort();

	let expected_document = Document {
		controller,
		authentication: AuthenticationMethod { controller: authentication },
		assertion_method: Some(AssertionMethod { controller: assertion }),
		services: services_keys,
	};

	assert_ok!(DID::create_did(origin, controller, authentication, Some(assertion), services));
	expected_document
}

fn hash_services(
	services: &BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices>,
) -> ServiceKeysOf<Test> {
	let mut services_keys: ServiceKeysOf<Test> = BoundedVec::default();
	for service in services {
		services_keys.try_push(<mock::Test as frame_system::Config>::Hashing::hash_of(&service));
	}
	services_keys
}

fn assert_services(
	services_info: BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices>,
	expected_consumers: u32,
) {
	let services_keys = hash_services(&services_info);

	for (i, key) in services_keys.iter().enumerate() {
		let service = DID::services(key).unwrap();
		assert_eq!(service.consumers(), expected_consumers);
		assert_eq!(service.info, services_info[i]);
	}
}

fn assert_services_do_not_exist(
	services_info: BoundedVec<ServiceInfo<Test>, <mock::Test as pallet::Config>::MaxServices>,
) {
	let services_keys = hash_services(&services_info);

	for key in services_keys {
		assert_eq!(DID::services(key), None);
	}
}

// ** DID Document Tests Works **

#[test]
fn create_did_works() {
	new_test_ext().execute_with(|| {
		// inserts default DID into storage. Checks for Ok()
		let expected_document = create_default_did(ALICE, ALICE);

		assert_eq!(Balances::reserved_balance(&ALICE), DidDeposit::get());
		assert_eq!(DID::dids(ALICE), Some(expected_document.clone()));
		assert_services(default_services(), 1);
		assert!(events()
			.contains(&Event::<Test>::DidCreated { did: ALICE, document: expected_document }));
	});
}

#[test]
fn update_did_works() {
	new_test_ext().execute_with(|| {
		let mut old_document = create_default_did(ALICE, ALICE);

		let origin = RuntimeOrigin::signed(ALICE);
		let controller = 2;
		let authentication: H160 = H160::from([1u8; 20]);
		let assertion: H160 = H160::from([1u8; 20]);
		let mut services = default_services();
		services[0].service_endpoint = bounded_vec![b's', b'3'];
		services[1].service_endpoint = bounded_vec![b's', b'4'];
		services[2].service_endpoint = bounded_vec![b's', b'5'];

		let mut services_keys = hash_services(&services);
		services_keys.sort();

		let expected_document = Document {
			controller,
			authentication: AuthenticationMethod { controller: authentication },
			assertion_method: Some(AssertionMethod { controller: assertion }),
			services: services_keys,
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
		assert!(events()
			.contains(&Event::<Test>::DidUpdated { did: ALICE, document: expected_document }));
	});
}

#[test]
fn force_update_did_works() {
	new_test_ext().execute_with(|| {
		// default did with origin & did for Alice
		let mut old_document = create_default_did(ALICE, ALICE);

		let origin = RuntimeOrigin::root();
		let controller = 2;
		let authentication: H160 = H160::from([1u8; 20]);
		let assertion: H160 = H160::from([1u8; 20]);
		let mut services = default_services();
		services[0].service_endpoint = bounded_vec![b's', b'3'];
		services[1].service_endpoint = bounded_vec![b's', b'4'];
		services[2].service_endpoint = bounded_vec![b's', b'5'];

		let mut services_keys = hash_services(&services);
		services_keys.sort();

		let expected_document = Document {
			controller,
			authentication: AuthenticationMethod { controller: authentication },
			assertion_method: Some(AssertionMethod { controller: assertion }),
			services: services_keys,
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
		let expected_document = create_default_did(ALICE, ALICE);

		assert_ok!(DID::remove_did(origin, ALICE));
		assert_eq!(DID::dids(ALICE), None);
		assert_eq!(Balances::reserved_balance(&ALICE), 0);
		assert_services_do_not_exist(default_services());
		assert!(events().contains(&Event::<Test>::DidRemoved { did: ALICE }));
	});
}

#[test]
fn force_remove_did_works() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::root();
		// inserts default DID into storage. Checks for Ok()
		let expected_document = create_default_did(ALICE, ALICE);

		assert_ok!(DID::force_remove_did(origin, ALICE));
		assert_eq!(DID::dids(ALICE), None);
		assert_eq!(Balances::reserved_balance(&ALICE), 0);
		assert_services_do_not_exist(default_services());
		assert!(events().contains(&Event::<Test>::DidForcedRemoved { did: ALICE }));
	});
}

#[test]
fn add_did_services_works() {
	new_test_ext().execute_with(|| {
		let mut old_document = create_default_did(ALICE, ALICE);

		let origin = RuntimeOrigin::signed(ALICE);

		let mut new_services = default_services();
		new_services[0].service_endpoint = bounded_vec![b's', b'3'];
		new_services[1].service_endpoint = bounded_vec![b's', b'4'];
		new_services[2].service_endpoint = bounded_vec![b's', b'5'];

		let mut new_services_keys = hash_services(&new_services);
		new_services_keys.sort();

		let mut combined_services = old_document.services.clone();
		new_services_keys.clone().into_iter().for_each(|key| {
			combined_services.try_push(key);
		});
		combined_services.sort();

		let expected_document = Document { services: combined_services, ..old_document };

		assert_ok!(DID::add_did_services(origin, ALICE, new_services.clone()));
		assert_eq!(DID::dids(ALICE), Some(expected_document.clone()));
		// assert services exist and have a consumer count of 1
		assert_services(new_services.clone(), 1);

		// assert that the default services were removed from storage
		assert!(events().contains(&Event::<Test>::DidServicesAdded {
			did: ALICE,
			new_services: new_services_keys,
		}));
	});
}

#[test]
fn remove_did_services_works() {
	new_test_ext().execute_with(|| {
		let mut old_document = create_default_did(ALICE, ALICE);

		let origin = RuntimeOrigin::signed(ALICE);

		let mut service_remaining = default_services();
		// remove last two services from `service_remaining`, leaving only one in `service_remaining`
		let mut services_to_remove =
			bounded_vec![service_remaining.pop().unwrap(), service_remaining.pop().unwrap()];

		let remaining_key = hash_services(&service_remaining);
		let mut to_remove_keys = hash_services(&services_to_remove);
		to_remove_keys.sort();

		let expected_document = Document { services: remaining_key.clone(), ..old_document };

		assert_ok!(DID::remove_did_services(origin, ALICE, to_remove_keys.clone()));
		assert_eq!(DID::dids(ALICE), Some(expected_document.clone()));
		// assert remaining service exists and has a consumer count of 1
		assert_services(service_remaining, 1);
		assert_services_do_not_exist(services_to_remove);

		// assert that the default services were removed from storage
		assert!(events().contains(&Event::<Test>::DidServicesRemoved {
			did: ALICE,
			removed_services: to_remove_keys,
		}));
	});
}

#[test]
fn multiple_service_consumers_works() {
	new_test_ext().execute_with(|| {
		let _ = create_default_did(ALICE, ALICE);
		let _ = create_default_did(BOB, BOB);

		assert_services(default_services(), 2);

		let mut services_to_remove = hash_services(&default_services());

		assert_ok!(DID::remove_did_services(
			RuntimeOrigin::signed(ALICE),
			ALICE,
			services_to_remove.clone()
		));
		assert_services(default_services(), 1);

		assert_ok!(DID::remove_did_services(
			RuntimeOrigin::signed(BOB),
			BOB,
			services_to_remove.clone()
		));

		assert_services_do_not_exist(default_services());
	});
}

//  ** DID Document Tests Fail Successfully **

#[test]
fn did_not_found_fails_for_all() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let gov_origin = RuntimeOrigin::root();

		assert_noop!(
			DID::update_did(origin.clone(), ALICE, None, None, None, None),
			Error::<Test>::DidNotFound
		);
		assert_noop!(
			DID::force_update_did(gov_origin.clone(), ALICE, None, None, None, None),
			Error::<Test>::DidNotFound
		);
		assert_noop!(DID::remove_did(origin.clone(), ALICE), Error::<Test>::DidNotFound);
		assert_noop!(DID::force_remove_did(gov_origin.clone(), ALICE), Error::<Test>::DidNotFound);
		assert_noop!(
			DID::add_did_services(origin.clone(), ALICE, BoundedVec::default()),
			Error::<Test>::DidNotFound
		);
		assert_noop!(
			DID::remove_did_services(origin.clone(), ALICE, BoundedVec::default()),
			Error::<Test>::DidNotFound
		);
	});
}

#[test]
fn not_did_controller_fails_for_all() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let gov_origin = RuntimeOrigin::root();

		// Create DID for Alice with Bob as controller
		let expected_document = create_default_did(ALICE, BOB);

		assert_noop!(
			DID::update_did(origin.clone(), ALICE, None, None, None, None),
			Error::<Test>::NotController
		);
		assert_noop!(DID::remove_did(origin.clone(), ALICE), Error::<Test>::NotController);
		assert_noop!(
			DID::add_did_services(origin.clone(), ALICE, BoundedVec::default()),
			Error::<Test>::NotController
		);
		assert_noop!(
			DID::remove_did_services(origin.clone(), ALICE, BoundedVec::default()),
			Error::<Test>::NotController
		);
	});
}

#[test]
fn force_fails_if_not_governance() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);

		let expected_document = create_default_did(ALICE, ALICE);

		assert_noop!(
			DID::force_update_did(origin.clone(), ALICE, None, None, None, None),
			BadOrigin
		);
		assert_noop!(DID::force_remove_did(origin.clone(), ALICE), BadOrigin);
	});
}

#[test]
fn create_did_fails_if_did_already_exists() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let controller = ALICE;
		let authentication: H160 = H160::from([0u8; 20]);
		let mut services = default_services();

		// Create DID for Alice
		let expected_document = create_default_did(ALICE, ALICE);

		assert_noop!(
			DID::create_did(
				origin.clone(),
				controller,
				authentication,
				None,
				BoundedVec::default()
			),
			Error::<Test>::DidAlreadyExists
		);
	});
}

#[test]
fn create_did_fails_if_account_does_not_have_enough_funds() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ACCOUNT_00);
		let controller = ACCOUNT_00;
		let authentication: H160 = H160::from([0u8; 20]);

		assert_noop!(
			DID::create_did(origin, controller, authentication, None, BoundedVec::default()),
			pallet_balances::Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn create_did_fails_if_duplicated_service() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let controller = 1;
		let authentication: H160 = H160::from([0u8; 20]);
		let mut services = default_services();

		services[1].service_endpoint = bounded_vec![b's', b'0'];
		assert_noop!(
			DID::create_did(origin, controller, authentication, None, services),
			Error::<Test>::ServiceAlreadyInDid
		);
	});
}

#[test]
fn update_did_fails_if_duplicated_service() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let mut services = default_services();

		let expected_document = create_default_did(ALICE, ALICE);

		// duplicate service
		services[1].service_endpoint = bounded_vec![b's', b'0'];

		assert_noop!(
			DID::update_did(origin, ALICE, None, None, None, Some(services)),
			Error::<Test>::ServiceAlreadyInDid
		);
	});
}

#[test]
fn force_update_did_fails_if_duplicated_service() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::root();
		let mut services = default_services();

		let expected_document = create_default_did(ALICE, ALICE);

		// duplicate service
		services[1].service_endpoint = bounded_vec![b's', b'0'];

		assert_noop!(
			DID::force_update_did(origin, ALICE, None, None, None, Some(services)),
			Error::<Test>::ServiceAlreadyInDid
		);
	});
}

#[test]
fn add_did_services_fails_if_too_many_services() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let mut services = default_services();

		create_default_did(ALICE, ALICE);

		// modify the 3 services from default_services()
		for i in 0..services.len() {
			services[i].service_endpoint = bounded_vec![b'o', b'0' + i as u8];
		}

		// insert max amount of services (with incremented indexes)
		for i in 0..(<mock::Test as pallet::Config>::MaxServices::get() - services.len() as u8) {
			services.try_push(ServiceInfo {
				type_id: types::ServiceType::VerifiableCredentialFileStorage,
				service_endpoint: bounded_vec![b'm', b'0' + i],
			});
		}

		// ensure services are not added. MaxServices + default_services().len() == out of limit
		assert_noop!(
			DID::add_did_services(origin, ALICE, services),
			Error::<Test>::TooManyServicesInDid
		);
	});
}

#[test]
fn add_did_services_fails_if_duplicated_service() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let mut services = default_services();

		let expected_document = create_default_did(ALICE, ALICE);

		// duplicate service
		services[1].service_endpoint = bounded_vec![b's', b'0'];

		assert_noop!(
			DID::add_did_services(origin, ALICE, services),
			Error::<Test>::ServiceAlreadyInDid
		);
	});
}

#[test]
fn remove_did_services_fails_if_service_not_in_did() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(ALICE);
		let mut services = default_services();

		let expected_document = create_default_did(ALICE, ALICE);

		// service that does not exist
		services[0].service_endpoint = bounded_vec![b'd', b'0'];

		let to_remove_keys = hash_services(&services);

		assert_noop!(
			DID::remove_did_services(origin, ALICE, to_remove_keys),
			Error::<Test>::ServiceNotInDid
		);
	});
}

// ** Issuer Tests **

#[test]
fn add_issuer_works() {
	new_test_ext().execute_with(|| {
		let issuer_info = Issuers::<Test>::get(ACCOUNT_01);
		assert_eq!(issuer_info, None);

		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_01));

		let issuer_info = Issuers::<Test>::get(ACCOUNT_01).unwrap();
		assert_eq!(issuer_info.status, IssuerStatus::Active);

		assert_noop!(
			DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_01),
			Error::<Test>::IssuerAlreadyExists
		);

		assert_noop!(DID::add_issuer(RuntimeOrigin::signed(1), ACCOUNT_01), BadOrigin);

		let events = events();
		assert!(events.contains(&Event::<Test>::IssuerStatusActive { issuer: ACCOUNT_01 }));
	});
}

#[test]
fn revoke_issuer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_01));
		assert_ok!(DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_01));

		let issuer_info = Issuers::<Test>::get(ACCOUNT_01).unwrap();
		assert_eq!(issuer_info.status, IssuerStatus::Revoked);

		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_02));

		assert_noop!(DID::revoke_issuer(RuntimeOrigin::signed(1), ACCOUNT_02), BadOrigin);

		assert_ok!(DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_02));

		assert_noop!(
			DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_02),
			Error::<Test>::IssuerNotActive
		);

		assert_noop!(
			DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_03),
			Error::<Test>::IssuerDoesNotExist
		);

		let events = events();
		assert!(events.contains(&Event::<Test>::IssuerStatusRevoked { issuer: ACCOUNT_01 }));
	});
}

#[test]
fn reactivate_issuer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_01));
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_02));
		assert_ok!(DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_01));

		assert_noop!(DID::reactivate_issuer(RuntimeOrigin::signed(1), ACCOUNT_01), BadOrigin);

		assert_ok!(DID::reactivate_issuer(RuntimeOrigin::root(), ACCOUNT_01));

		let issuer_info = Issuers::<Test>::get(ACCOUNT_01).unwrap();
		assert_eq!(issuer_info.status, IssuerStatus::Active);

		assert_noop!(
			DID::reactivate_issuer(RuntimeOrigin::root(), ACCOUNT_02),
			Error::<Test>::IssuerNotRevoked
		);

		assert_noop!(
			DID::reactivate_issuer(RuntimeOrigin::root(), ACCOUNT_03),
			Error::<Test>::IssuerDoesNotExist
		);

		let events = events();
		assert!(events.contains(&Event::<Test>::IssuerStatusReactived { issuer: ACCOUNT_01 }));
	});
}

#[test]
fn remove_issuer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_01));
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_02));
		assert_ok!(DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_01));

		assert_noop!(DID::remove_issuer(RuntimeOrigin::signed(1), ACCOUNT_01), BadOrigin);

		assert_ok!(DID::remove_issuer(RuntimeOrigin::root(), ACCOUNT_01));

		let issuer_info = Issuers::<Test>::get(ACCOUNT_01);
		assert_eq!(issuer_info, None);

		assert_noop!(
			DID::remove_issuer(RuntimeOrigin::root(), ACCOUNT_02),
			Error::<Test>::IssuerNotRevoked
		);

		assert_noop!(
			DID::remove_issuer(RuntimeOrigin::root(), ACCOUNT_03),
			Error::<Test>::IssuerDoesNotExist
		);

		let events = events();
		assert!(events.contains(&Event::<Test>::IssuerRemoved { issuer: ACCOUNT_01 }));
	});
}

#[test]
fn add_credentials_type_works() {
	new_test_ext().execute_with(|| {
		let creds: Vec<BoundedVec<u8, MaxString>> =
			vec![bounded_vec![0, 0], bounded_vec![0, 1], bounded_vec![0, 2]];

		assert_noop!(DID::add_credentials_type(RuntimeOrigin::signed(1), creds.clone()), BadOrigin);

		assert_ok!(DID::add_credentials_type(RuntimeOrigin::root(), creds.clone()));

		assert_noop!(
			DID::add_credentials_type(RuntimeOrigin::root(), creds.clone()),
			Error::<Test>::CredentialTypeAlreadyAdded
		);

		let mut max_creds: Vec<BoundedVec<u8, MaxString>> = vec![];
		let creds_limit = MaxCredentialsTypes::get();
		for i in 3..creds_limit {
			max_creds.push(bounded_vec![0, i]);
		}

		assert_ok!(DID::add_credentials_type(RuntimeOrigin::root(), max_creds.clone()));

		assert_noop!(
			DID::add_credentials_type(
				RuntimeOrigin::root(),
				vec![bounded_vec![0, creds_limit + 1]]
			),
			Error::<Test>::MaxCredentials
		);

		let events = events();
		assert!(events.contains(&Event::<Test>::CredentialTypesAdded { credentials: creds }));
	});
}

#[test]
fn remove_credentials_type_works() {
	new_test_ext().execute_with(|| {
		let cred_x: Vec<BoundedVec<u8, MaxString>> = vec![bounded_vec![0, 1], bounded_vec![0, 2]];
		assert_ok!(DID::add_credentials_type(RuntimeOrigin::root(), cred_x.clone()));

		assert_noop!(
			DID::remove_credentials_type(RuntimeOrigin::signed(1), cred_x.clone()),
			BadOrigin
		);

		assert_ok!(DID::remove_credentials_type(RuntimeOrigin::root(), cred_x.clone()));

		let events = events();
		assert!(events.contains(&Event::<Test>::CredentialTypesRemoved { credentials: cred_x }));

		let cred_y: Vec<BoundedVec<u8, MaxString>> = vec![bounded_vec![0, 2], bounded_vec![0, 4]];
		assert_noop!(
			DID::remove_credentials_type(RuntimeOrigin::root(), cred_y),
			Error::<Test>::CredentialTypeDoesNotExist
		);

		let cred_x: Vec<BoundedVec<u8, MaxString>> = vec![bounded_vec![0, 1]];
		let cred_y: Vec<BoundedVec<u8, MaxString>> = vec![bounded_vec![0, 2]];
		let cred_z: Vec<BoundedVec<u8, MaxString>> = vec![bounded_vec![0, 4], bounded_vec![0, 3]];
		let ordered_creds: Vec<BoundedVec<u8, MaxString>> =
			vec![bounded_vec![0, 1], bounded_vec![0, 2], bounded_vec![0, 3], bounded_vec![0, 4]];

		assert_ok!(DID::add_credentials_type(RuntimeOrigin::root(), cred_z.clone()));
		assert_ok!(DID::add_credentials_type(RuntimeOrigin::root(), cred_y.clone()));
		assert_ok!(DID::add_credentials_type(RuntimeOrigin::root(), cred_x.clone()));
		assert_eq!(CredentialsTypes::<Test>::get(), ordered_creds);
	});
}

#[test]
fn issue_credentials_works() {
	new_test_ext().execute_with(|| {
		let issuer_origin = RuntimeOrigin::signed(ACCOUNT_01);
		let root = RuntimeOrigin::root();

		create_default_did(ACCOUNT_01, ACCOUNT_01);
		create_default_did(ACCOUNT_02, ACCOUNT_02);
		create_default_did(ACCOUNT_04, ACCOUNT_04);

		let creds: Vec<BoundedVec<u8, MaxString>> =
			vec![bounded_vec![0, 0], bounded_vec![0, 1], bounded_vec![0, 2]];
		let verifiable_credential_hash: HashOf<Test> = bounded_vec![1, 2, 3, 4, 5];

		assert_ok!(DID::add_credentials_type(root.clone(), creds.clone()));
		assert_ok!(DID::add_issuer(root.clone(), ACCOUNT_01));

		assert_ok!(DID::add_issuer(root.clone(), ACCOUNT_04));
		assert_ok!(DID::revoke_issuer(root.clone(), ACCOUNT_04));

		assert_noop!(
			DID::issue_credentials(
				RuntimeOrigin::signed(ACCOUNT_02),
				ACCOUNT_02,
				ACCOUNT_02,
				creds.clone(),
				verifiable_credential_hash.clone()
			),
			Error::<Test>::NotIssuer
		);
		assert_noop!(
			DID::issue_credentials(
				RuntimeOrigin::signed(ACCOUNT_02),
				ACCOUNT_01,
				ACCOUNT_02,
				creds.clone(),
				verifiable_credential_hash.clone()
			),
			Error::<Test>::NotController
		);
		assert_noop!(
			DID::issue_credentials(
				issuer_origin.clone(),
				ACCOUNT_03,
				ACCOUNT_03,
				creds.clone(),
				verifiable_credential_hash.clone()
			),
			Error::<Test>::DidNotFound
		);
		assert_noop!(
			DID::issue_credentials(
				issuer_origin.clone(),
				ACCOUNT_01,
				ACCOUNT_03,
				creds.clone(),
				verifiable_credential_hash.clone()
			),
			Error::<Test>::DidNotFound
		);
		assert_noop!(
			DID::issue_credentials(
				RuntimeOrigin::signed(ACCOUNT_04),
				ACCOUNT_04,
				ACCOUNT_02,
				creds.clone(),
				verifiable_credential_hash.clone()
			),
			Error::<Test>::IssuerNotActive
		);

		assert_ok!(DID::issue_credentials(
			issuer_origin.clone(),
			ACCOUNT_01,
			ACCOUNT_02,
			creds.clone(),
			verifiable_credential_hash.clone()
		));

		for cred in creds.iter() {
			assert_eq!(
				DID::issued_credentials((ACCOUNT_02, cred.clone(), ACCOUNT_01)),
				Some(CredentialInfo {
					verifiable_credential_hash: verifiable_credential_hash.clone()
				})
			);
		}

		let events = events();
		assert!(events.contains(&Event::<Test>::CredentialsIssued {
			issuer: ACCOUNT_01,
			did: ACCOUNT_02,
			credentials: creds,
			verifiable_credential_hash
		}));
	});
}

#[test]
fn revoke_credentials_works() {
	new_test_ext().execute_with(|| {
		let issuer_origin = RuntimeOrigin::signed(ACCOUNT_01);
		let root = RuntimeOrigin::root();

		create_default_did(ACCOUNT_01, ACCOUNT_01);
		create_default_did(ACCOUNT_02, ACCOUNT_02);

		let creds: Vec<BoundedVec<u8, MaxString>> =
			vec![bounded_vec![0, 0], bounded_vec![0, 1], bounded_vec![0, 2]];
		let verifiable_credential_hash: HashOf<Test> = bounded_vec![1, 2, 3, 4, 5];

		assert_ok!(DID::add_credentials_type(root.clone(), creds.clone()));
		assert_ok!(DID::add_issuer(root.clone(), ACCOUNT_01));

		assert_ok!(DID::issue_credentials(
			issuer_origin.clone(),
			ACCOUNT_01,
			ACCOUNT_02,
			creds.clone(),
			verifiable_credential_hash.clone()
		));

		assert_noop!(
			DID::revoke_credentials(
				RuntimeOrigin::signed(ACCOUNT_02),
				ACCOUNT_01,
				ACCOUNT_02,
				creds.clone(),
			),
			Error::<Test>::NotController
		);
		assert_noop!(
			DID::revoke_credentials(issuer_origin.clone(), ACCOUNT_03, ACCOUNT_03, creds.clone(),),
			Error::<Test>::DidNotFound
		);
		assert_noop!(
			DID::revoke_credentials(
				issuer_origin.clone(),
				ACCOUNT_01,
				ACCOUNT_02,
				vec![bounded_vec![9, 9]],
			),
			Error::<Test>::IssuedCredentialDoesNotExist
		);

		assert_ok!(DID::revoke_credentials(
			issuer_origin.clone(),
			ACCOUNT_01,
			ACCOUNT_02,
			creds.clone(),
		));

		for cred in creds.iter() {
			assert_eq!(DID::issued_credentials((ACCOUNT_02, cred.clone(), ACCOUNT_01)), None);
		}

		let events = events();
		assert!(events.contains(&Event::<Test>::CredentialsRevoked {
			issuer: ACCOUNT_01,
			did: ACCOUNT_02,
			credentials: creds,
		}));
	});
}
