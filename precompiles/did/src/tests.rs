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
use frame_support::{assert_ok, sp_runtime::traits::Hash};
use pallet_did::{
	types::{AssertionMethod, AuthenticationMethod, Document},
	ServiceKeysOf,
};
use precompile_utils::testing::PrecompileTesterExt;
use sp_core::{bounded_vec, H160};

use super::*;
use crate::mock::*;

fn precompiles() -> TestPrecompileSet<Test> {
	PrecompilesValue::get()
}

fn events() -> Vec<pallet_did::Event<Test>> {
	let result = System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let mock::RuntimeEvent::DID(inner) = e { Some(inner) } else { None })
		.collect::<Vec<_>>();

	System::reset_events();

	result
}

fn hash_services(
	services: &BoundedVec<ServiceInfo<Test>, <mock::Test as pallet_did::Config>::MaxServices>,
) -> ServiceKeysOf<Test> {
	let mut services_keys: ServiceKeysOf<Test> = BoundedVec::default();
	for service in services {
		let _ = services_keys
			.try_push(<mock::Test as frame_system::Config>::Hashing::hash_of(&service));
	}
	services_keys
}

fn create_default_did(controller: TestAccount, use_assertion: bool) -> Document<Test> {
	let controller = controller;
	let authentication: H160 = H160::from([0u8; 20]);
	let assertion: H160 = H160::from([1u8; 20]);
	let services = default_services();
	let mut services_keys = hash_services(&services);
	services_keys.sort();

	let expected_document = Document {
		controller,
		authentication: AuthenticationMethod { controller: authentication },
		assertion_method: if use_assertion {
			Some(AssertionMethod { controller: assertion })
		} else {
			None
		},
		services: services_keys,
	};
	expected_document
}

fn insert_default_did(controller: TestAccount) {
	let did = create_default_did(controller.clone(), true);
	let origin = RuntimeOrigin::signed(controller.clone());
	let assertion = match did.assertion_method {
		Some(address) => Some(address.controller),
		None => None,
	};
	assert_ok!(DID::create_did(
		origin,
		did.controller,
		did.authentication.controller,
		assertion,
		default_services()
	));
	assert!(DID::dids::<TestAccount>(controller).is_some());
}

fn default_services(
) -> BoundedVec<ServiceInfo<Test>, <mock::Test as pallet_did::Config>::MaxServices> {
	bounded_vec![ServiceInfo {
		type_id: pallet_did::types::ServiceType::VerifiableCredentialFileStorage,
		service_endpoint: bounded_vec![b's', b'0']
	},]
}

#[test]
fn it_creates_did_without_assertion() {
	new_test_ext().execute_with(|| {
		let expected_document = create_default_did(TestAccount::Alice, false);
		precompiles()
			.prepare_test(
				TestAccount::Alice,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::CreateDID)
					.write(Address(TestAccount::Alice.into()))
					.write(Address(H160::from([0u8; 20])))
					.write((false, Address(H160::from([0u8; 20]))))
					.write(vec![(1u8, Bytes(default_services()[0].service_endpoint.to_vec()))])
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		let events = events();
		assert!(events.contains(&pallet_did::Event::<Test>::DidCreated {
			did: TestAccount::Alice,
			document: expected_document,
		}));
	});
}

#[test]
fn it_creates_did_with_assertion() {
	new_test_ext().execute_with(|| {
		let expected_document = create_default_did(TestAccount::Alice, true);
		precompiles()
			.prepare_test(
				TestAccount::Alice,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::CreateDID)
					.write(Address(TestAccount::Alice.into()))
					.write(Address(H160::from([0u8; 20])))
					.write((true, Address(H160::from([1u8; 20]))))
					.write(vec![(0u8, Bytes(default_services()[0].service_endpoint.to_vec()))])
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert!(events().contains(&pallet_did::Event::<Test>::DidCreated {
			did: TestAccount::Alice,
			document: expected_document,
		}));
	});
}

#[test]
fn it_updates_nothing_from_did() {
	new_test_ext().execute_with(|| {
		let expected_document = create_default_did(TestAccount::Alice, true);
		insert_default_did(TestAccount::Alice);
		precompiles()
			.prepare_test(
				TestAccount::Alice,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::UpdateDID)
					.write(Address(TestAccount::Alice.into()))
					.write((false, Address(TestAccount::Alice.into())))
					.write((false, Address(H160::from([0u8; 20]))))
					.write((false, Address(H160::from([0u8; 20]))))
					.write((
						false,
						vec![(0u8, Bytes(default_services()[0].service_endpoint.to_vec()))],
					))
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert!(events().contains(&pallet_did::Event::<Test>::DidUpdated {
			did: TestAccount::Alice,
			document: expected_document,
		}));
	});
}

#[test]
fn it_updates_all_from_did() {
	new_test_ext().execute_with(|| {
		let expected_document = Document {
			controller: TestAccount::Bob,
			authentication: AuthenticationMethod { controller: H160::from([1u8; 20]) },
			assertion_method: Some(AssertionMethod { controller: H160::from([2u8; 20]) }),
			services: BoundedVec::default(),
		};
		insert_default_did(TestAccount::Alice);
		precompiles()
			.prepare_test(
				TestAccount::Alice,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::UpdateDID)
					.write(Address(TestAccount::Alice.into()))
					.write((true, Address(TestAccount::Bob.into())))
					.write((true, Address(H160::from([1u8; 20]))))
					.write((true, Address(H160::from([2u8; 20]))))
					.write((true, vec![] as Vec<(bool, Vec<(u8, Bytes)>)>))
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert!(events().contains(&pallet_did::Event::<Test>::DidUpdated {
			did: TestAccount::Alice,
			document: expected_document,
		}));
	});
}

#[test]
fn it_removes_a_did() {
	new_test_ext().execute_with(|| {
		insert_default_did(TestAccount::Bob);
		assert!(DID::dids::<TestAccount>(TestAccount::Bob).is_some());
		precompiles()
			.prepare_test(
				TestAccount::Bob,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::RemoveDID)
					.write(Address(TestAccount::Bob.into()))
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert!(events().contains(&pallet_did::Event::<Test>::DidRemoved { did: TestAccount::Bob }));
		assert!(DID::dids::<TestAccount>(TestAccount::Bob).is_none());
	});
}

#[test]
fn reverts_remove_did_if_not_controller() {
	new_test_ext().execute_with(|| {
		insert_default_did(TestAccount::Bob);
		assert!(DID::dids::<TestAccount>(TestAccount::Bob).is_some());
		precompiles()
			.prepare_test(
				TestAccount::Alice,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::RemoveDID)
					.write(Address(TestAccount::Bob.into()))
					.build(),
			)
			.execute_reverts(|_| {
				//todo: proper revert check
				true
			});
		assert!(DID::dids::<TestAccount>(TestAccount::Bob).is_some());
	});
}

#[test]
fn can_add_did_services() {
	new_test_ext().execute_with(|| {
		let services: BoundedVec<
			ServiceInfo<Test>,
			<mock::Test as pallet_did::Config>::MaxServices,
		> = bounded_vec![
			ServiceInfo {
				type_id: pallet_did::types::ServiceType::VerifiableCredentialFileStorage,
				service_endpoint: bounded_vec![b's', b'1']
			},
			ServiceInfo {
				type_id: pallet_did::types::ServiceType::VerifiableCredentialFileStorage,
				service_endpoint: bounded_vec![b's', b'2']
			},
		];
		insert_default_did(TestAccount::Alice);
		let mut service_keys = hash_services(&services);
		service_keys.sort();
		let mut raw_services: Vec<(u8, Bytes)> = Vec::with_capacity(services.len());

		for service in services.iter() {
			match service.type_id {
				ServiceType::VerifiableCredentialFileStorage =>
					raw_services.push((0u8, Bytes(service.service_endpoint.to_vec()))),
			}
		}

		precompiles()
			.prepare_test(
				TestAccount::Alice,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::AddDIDServices)
					.write(Address(TestAccount::Alice.into()))
					.write::<Vec<(u8, Bytes)>>(raw_services)
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert!(events().contains(&pallet_did::Event::<Test>::DidServicesAdded {
			did: TestAccount::Alice,
			new_services: service_keys,
		}));
	});
}

#[test]
fn can_remove_did_services() {
	new_test_ext().execute_with(|| {
		insert_default_did(TestAccount::Charlie);
		let services_keys = hash_services(&default_services());
		precompiles()
			.prepare_test(
				TestAccount::Charlie,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::RemoveDIDServices)
					.write(Address(TestAccount::Charlie.into()))
					.write::<Vec<H256>>(services_keys.to_vec())
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert!(events().contains(&pallet_did::Event::<Test>::DidServicesRemoved {
			did: TestAccount::Charlie,
			removed_services: services_keys,
		}));
	});
}
