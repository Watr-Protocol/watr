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
	assert_noop, assert_ok, bounded_vec, dispatch::GetDispatchInfo, weights::Weight,
};
use frame_system::{EventRecord, Phase};
use mock::{RuntimeCall, RuntimeEvent};
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash};

fn record(event: RuntimeEvent) -> EventRecord<RuntimeEvent, H256> {
	EventRecord { phase: Phase::Initialization, event, topics: vec![] }
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
		assert_ok!(DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_02));

		assert_noop!(
			DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_02),
			Error::<Test>::IssuerNotActive
		);

		assert_noop!(
			DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_03),
			Error::<Test>::IssuerDoesNotExist
		);
	});
}

#[test]
fn reactivate_issuer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_01));
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_02));
		assert_ok!(DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_01));
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
	});
}

#[test]
fn remove_issuer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_01));
		assert_ok!(DID::add_issuer(RuntimeOrigin::root(), ACCOUNT_02));
		assert_ok!(DID::revoke_issuer(RuntimeOrigin::root(), ACCOUNT_01));
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
	});
}

#[test]
fn add_credential_type_works() {
	new_test_ext().execute_with(|| {
		let cred: BoundedVec<u8, MaxString> = bounded_vec![0, 1];
		assert_ok!(DID::add_credential_type(RuntimeOrigin::root(), cred.clone()));

		assert_noop!(
			DID::add_credential_type(RuntimeOrigin::root(), cred),
			Error::<Test>::CredentialAlreadyAdded
		);
	});
}

#[test]
fn remove_credential_type_works() {
	new_test_ext().execute_with(|| {
		let cred_x: BoundedVec<u8, MaxString> = bounded_vec![0, 1];
		assert_ok!(DID::add_credential_type(RuntimeOrigin::root(), cred_x.clone()));

		assert_ok!(DID::remove_credential_type(RuntimeOrigin::root(), cred_x));

		let cred_y: BoundedVec<u8, MaxString> = bounded_vec![0, 2];
		assert_noop!(
			DID::remove_credential_type(RuntimeOrigin::root(), cred_y),
			Error::<Test>::CredentialDoesNotExist
		);

		let cred_x: BoundedVec<u8, MaxString> = bounded_vec![0, 1];
		let cred_y: BoundedVec<u8, MaxString> = bounded_vec![0, 2];
		let cred_z: BoundedVec<u8, MaxString> = bounded_vec![0, 4];
		let ordered_creds: BoundedVec<BoundedVec<u8, MaxString>, MaxCredentialsTypes> =
			bounded_vec![cred_x.clone(), cred_y.clone(), cred_z.clone()];

		assert_ok!(DID::add_credential_type(RuntimeOrigin::root(), cred_z.clone()));
		assert_ok!(DID::add_credential_type(RuntimeOrigin::root(), cred_y.clone()));
		assert_ok!(DID::add_credential_type(RuntimeOrigin::root(), cred_x.clone()));
		assert_eq!(CredentialsTypes::<Test>::get(), ordered_creds)
	});
}
