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

use frame_support::sp_runtime::traits::Hash;
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
	let assertion: H160 = H160::from([0u8; 20]);
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

fn default_services(
) -> BoundedVec<ServiceInfo<Test>, <mock::Test as pallet_did::Config>::MaxServices> {
	bounded_vec![ServiceInfo {
		type_id: pallet_did::types::ServiceType::VerifiableCredentialFileStorage,
		service_endpoint: bounded_vec![b's', b'0']
	},]
}

#[test]
fn it_creates_did() {
	new_test_ext().execute_with(|| {
		let expected_document = create_default_did(TestAccount::Alice, false);
		precompiles()
			.prepare_test(
				TestAccount::Alice,
				PRECOMPILE_ADDRESS,
				EvmDataWriter::new_with_selector(Action::CreateDID)
					.write(Address(TestAccount::Alice.into()))
					.write(Address(H160::from([0u8; 20])))
					.write(Bytes(default_services()[0].service_endpoint.to_vec()))
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		let events = events();
		assert!(events.contains(&pallet_did::Event::<Test>::DidCreated {
			did: TestAccount::Alice,
			document: expected_document
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
				EvmDataWriter::new_with_selector(Action::CreateDIDOptional)
					.write(Address(TestAccount::Alice.into()))
					.write(Address(H160::from([0u8; 20])))
					.write(Address(H160::from([0u8; 20])))
					.write(Bytes(default_services()[0].service_endpoint.to_vec()))
					.build(),
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		let events = events();
		assert!(events.contains(&pallet_did::Event::<Test>::DidCreated {
			did: TestAccount::Alice,
			document: expected_document
		}));
	});
}
