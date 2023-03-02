use crate::*;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
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
