#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_runtime::DispatchResult;
use sp_std::prelude::*;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::{DispatchResult, *};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		type SimpleMajorityOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type SuperMajorityOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type UnanimousOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A SimpleMajority motion was executed. motion_result contains the call result
		DispatchSimpleMajority { motion_result: DispatchResult },
		/// A SuperMajority motion was executed. motion_result contains the call result
		DispatchSuperMajority { motion_result: DispatchResult },
		/// A Unanimous motion was executed. motion_result contains the call result
		DispatchUnanimous { motion_result: DispatchResult },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Ensures the simple majority is met and dispatches a call with `Root` origin.
		///
		/// # <weight>
		/// - O(1).
		/// - Limited storage reads.
		/// - One DB write (event).
		/// - Weight of derivative `call` execution + 10,000.
		/// # </weight>
		//TODO: add proper weight
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(dispatch_info.weight, dispatch_info.class)
		})]
		pub fn simple_majority(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResult {
			T::SimpleMajorityOrigin::ensure_origin(origin)?;

			let motion_result = Self::do_dispatch(call);
			Self::deposit_event(Event::DispatchSimpleMajority { motion_result });

			Ok(())
		}

		/// Ensures the super majority is met and dispatches a call with `Root` origin.
		///
		/// # <weight>
		/// - O(1).
		/// - Limited storage reads.
		/// - One DB write (event).
		/// - Weight of derivative `call` execution + 10,000.
		/// # </weight>
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(dispatch_info.weight, dispatch_info.class)
		})]
		pub fn super_majority(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResult {
			T::SuperMajorityOrigin::ensure_origin(origin)?;

			let motion_result = Self::do_dispatch(call);
			Self::deposit_event(Event::DispatchSuperMajority { motion_result });

			Ok(())
		}

		/// Ensures unanimous voting is met and dispatches a call with `Root` origin.
		///
		/// # <weight>
		/// - O(1).
		/// - Limited storage reads.
		/// - One DB write (event).
		/// - Weight of derivative `call` execution + 10,000.
		/// # </weight>
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(dispatch_info.weight, dispatch_info.class)
		})]
		pub fn unanimous(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResult {
			T::UnanimousOrigin::ensure_origin(origin)?;

			let motion_result = Self::do_dispatch(call);
			Self::deposit_event(Event::DispatchUnanimous { motion_result });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Helper to actually dispatch RuntimeCall.
		///
		/// Should only be called after the origin is ensured.
		///
		/// Returns the `DispatchResult` from the dispatchable call.
		fn do_dispatch(call: Box<<T as Config>::RuntimeCall>) -> DispatchResult {
			let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
			let motion_result = res.map(|_| ()).map_err(|e| e.error);
			motion_result
		}
	}
}