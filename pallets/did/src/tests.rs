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
use sp_core::H256;
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
			Error::<Test>::CredentialAlreadyAdded
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
			Error::<Test>::CredentialDoesNotExist
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
