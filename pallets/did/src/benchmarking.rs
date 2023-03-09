use crate::*;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use sp_core::H160;

use super::{Pallet as DID, *};

const SEED: u32 = 0;

fn controller<T: Config>(i: u32) -> DidIdentifierOf<T> {
	let account = account::<T::AccountId>("controller", i, SEED);
	T::DidIdentifier::from(account)
}

fn issuer<T: Config>(i: u32) -> DidIdentifierOf<T> {
	let account = account::<T::AccountId>("issuer", i, SEED);
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

fn create_credential_type<T: Config>(i: u8) -> BoundedVec<u8, T::MaxString> {
	let mut cred = BoundedVec::default();
	let cred_bytes = i.to_be_bytes();

	for b in cred_bytes {
		cred.try_push(b);
	}

	cred
}

benchmarks! {
	// ---------------------------------------------
	create_did {
		let m in 0 .. T::MaxServices::get();

		let mut services = BoundedVec::default();
		for i in 0 .. m {
			let service = create_service::<T>(i);
			services.try_push(service);
		}

		let caller: T::AccountId = whitelisted_caller();
		let origin = RawOrigin::Signed(caller.clone());

		T::Currency::make_free_balance_be(&caller, T::DidDeposit::get());

		let controller = controller::<T>(1);
		let authentication = authentication::<T>(2);
		let assertion = Some(assertion::<T>(3));
		let mut services_keys = BoundedVec::default();

		for service in &services {
			let key = T::Hashing::hash_of(&service);
			let pos = services_keys
				.binary_search(&key).err().unwrap();
			services_keys
				.try_insert(pos, key.clone());
		}

		let document = Document {
			controller: controller.clone(),
			authentication: AuthenticationMethod::<T> { controller: authentication.clone() },
			assertion_method: Some(AssertionMethod::<T> { controller: assertion.clone().unwrap() }),
			services: services_keys.clone(),
		};
	}: _(
			RawOrigin::Signed(caller.clone()),
			controller,
			authentication,
			assertion,
			services
		)
	verify {
		assert_eq!(Did::get(T::DidIdentifier::from(caller)), Some(document));
	}

	// // ---------------------------------------------
	// update_did {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// remove_did {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// add_did_service {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

	// // ---------------------------------------------
	// remove_did_service {
	// 	/* code to set the initial state */
	// }: {
	// 	/* code to test the function benchmarked */
	// }
	// verify {
	// 	/* optional verification */
	// 	assert_eq!(true, true)
	// }

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

	// ---------------------------------------------
	add_issuer {
		let issuer = issuer::<T>(1);
		let info = IssuerInfo { status: IssuerStatus::Active };
	}: _(
		RawOrigin::Root, issuer.clone()
	)
	verify {
		assert_eq!(Issuers::<T>::get(issuer), Some(info));
	}

	// ---------------------------------------------
	revoke_issuer {
		let issuer = issuer::<T>(1);
		let info = IssuerInfo { status: IssuerStatus::Revoked };
		DID::<T>::add_issuer(RawOrigin::Root.into(), issuer.clone());
	}: _(
		RawOrigin::Root, issuer.clone()
	)
	verify {
		assert_eq!(Issuers::<T>::get(issuer), Some(info));
	}

	// ---------------------------------------------
	reactivate_issuer {
		let issuer = issuer::<T>(1);
		let info = IssuerInfo { status: IssuerStatus::Active };
		DID::<T>::add_issuer(RawOrigin::Root.into(), issuer.clone());
		DID::<T>::revoke_issuer(RawOrigin::Root.into(), issuer.clone());
	}: _(
		RawOrigin::Root, issuer.clone()
	)
	verify {
		assert_eq!(Issuers::<T>::get(issuer), Some(info));
	}

	// ---------------------------------------------
	remove_issuer {
		let issuer = issuer::<T>(1);
		DID::<T>::add_issuer(RawOrigin::Root.into(), issuer.clone());
		DID::<T>::revoke_issuer(RawOrigin::Root.into(), issuer.clone());
	}: _(
		RawOrigin::Root, issuer.clone()
	)
	verify {
		assert_eq!(Issuers::<T>::get(issuer), None);
	}

	// ---------------------------------------------
	add_credentials_type {
		let m in 0 .. T::MaxCredentialsTypes::get();
		let mut credentials_types = BoundedVec::default();
		CredentialsTypes::<T>::put(credentials_types.clone());

		for i in 0 .. m {
			let mut credentials_types:
				frame_support::BoundedVec<
					frame_support::BoundedVec<u8, T::MaxString>,
					T::MaxCredentialsTypes
				> = BoundedVec::default();
			let cred = create_credential_type::<T>(i as u8);
			credentials_types.try_push(cred.clone());
		}

		CredentialsTypes::<T>::put(credentials_types.clone());
	}: _(
		RawOrigin::Root, credentials_types.to_vec().clone()
	)
	verify {
		assert_eq!(CredentialsTypes::<T>::get(), credentials_types);
	}

	// ---------------------------------------------
	remove_credentials_type {
		let m in 0 .. T::MaxCredentialsTypes::get();
		let mut credentials_types = BoundedVec::default();

		for i in 0 .. m {
			let mut credentials_types:
				frame_support::BoundedVec<
					frame_support::BoundedVec<u8, T::MaxString>,
					T::MaxCredentialsTypes
				> = BoundedVec::default();
			let cred = create_credential_type::<T>(i as u8);
			credentials_types.try_push(cred.clone());
		}

		CredentialsTypes::<T>::put(credentials_types.clone());
	}: _(
		RawOrigin::Root, credentials_types.to_vec().clone()
	)
	verify {
		assert_eq!(CredentialsTypes::<T>::get(), credentials_types);
	}
}

// impl_benchmark_test_suite!(
// 	MyPallet,
// 	crate::mock::new_test_ext(),
// 	crate::mock::Test,
// );
