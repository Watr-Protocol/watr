use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use sp_core::H160;

use super::{Pallet as Did, *};

const SEED: u32 = 0;

fn controller<T: Config>(i: u32) -> DidIdentifierOf<T> {
	let account = account::<T::AccountId>("controller", i, SEED);
	T::DidIdentifier::from(account)
}

fn issuer<T: Config>(i: u32) -> DidIdentifierOf<T> {
	let account = account::<T::AccountId>("issuer", i, SEED);
	T::DidIdentifier::from(account)
}

fn authenticator<T: Config>(i: u64) -> T::AuthenticationAddress
where <T as pallet::Config>::AuthenticationAddress: From<H160>
{
	let authenticator = H160::from_low_u64_be(i);
	T::AuthenticationAddress::from(authenticator)
}

fn assertion<T: Config>(i: u64) -> T::AssertionAddress
where <T as pallet::Config>::AssertionAddress: From<H160>
{
	let assertion = H160::from_low_u64_be(i);
	T::AssertionAddress::from(assertion)
}

fn create_service<T: Config>(i: u32) -> ServiceInfo<T> {
	let mut endpoint_base = BoundedVec::default();
	// let mut endpoint_base: &[u8] = b"https://";

	ServiceInfo {
		type_id: ServiceType::VerifiableCredentialFileStorage,
		// service_endpoint: endpoint_base.to_vec(),
		service_endpoint: endpoint_base,
	}
}

benchmarks! {
	// ---------------------------------------------
	create_did {
		let m in 0 .. T::MaxServices::get();

		// Set DID services
		let mut services: BoundedVec<ServiceInfo<T>, T::MaxServices> = BoundedVec::default();
		for i in 0 .. m {
			let service = create_service::<T>(i);
			services.push(service);
		}

		let caller: T::AccountId = whitelisted_caller();
		let origin = RawOrigin::Signed(caller.clone());
		// fund the caller to be able to reserve
		T::Currency::make_free_balance_be(&caller, (u32::max_value() / 100).into());
	}: _(
			RawOrigin::Signed(caller.clone()),
			controller::<T>(1),
			authenticator::<T>(2),
			// T::AuthenticationAddress::from(H160::from_low_u64_be(2)),
			Some(assertion(3)),
			services
		)
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	update_did {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	remove_did {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	add_did_service {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	remove_did_service {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	issue_credentials {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	revoke_credentials {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	add_issuer {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	revoke_issuer {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	reactivate_issuer {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	remove_issuer {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	add_credentials_type {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}

	// ---------------------------------------------
	remove_credentials_type {
		/* code to set the initial state */
	}: {
		/* code to test the function benchmarked */
	}
	verify {
		/* optional verification */
		assert_eq!(true, true)
	}
}

// impl_benchmark_test_suite!(
// 	MyPallet,
// 	crate::mock::new_test_ext(),
// 	crate::mock::Test,
// );
