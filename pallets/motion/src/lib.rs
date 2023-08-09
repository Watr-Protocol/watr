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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_runtime::DispatchResult;
use sp_std::prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::{DispatchResult, *};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

	#[pallet::pallet]
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
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(dispatch_info.weight, dispatch_info.class)
		})]
		#[pallet::call_index(0)]
		pub fn simple_majority(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			T::SimpleMajorityOrigin::ensure_origin(origin)?;

			let motion_result = Self::do_dispatch(call);
			Self::deposit_event(Event::DispatchSimpleMajority { motion_result });

			Ok(Pays::No.into())
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
		#[pallet::call_index(1)]
		pub fn super_majority(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			T::SuperMajorityOrigin::ensure_origin(origin)?;

			let motion_result = Self::do_dispatch(call);
			Self::deposit_event(Event::DispatchSuperMajority { motion_result });

			Ok(Pays::No.into())
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
		#[pallet::call_index(2)]
		pub fn unanimous(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			T::UnanimousOrigin::ensure_origin(origin)?;

			let motion_result = Self::do_dispatch(call);
			Self::deposit_event(Event::DispatchUnanimous { motion_result });

			Ok(Pays::No.into())
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
