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

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-10-2-102-127`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/watr-node
// benchmark
// pallet
// --chain=mainnet-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_utility
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./runtime/mainnet/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		Weight::from_ref_time(26_467_000 as u64)
			// Standard Error: 43_652
			.saturating_add(Weight::from_ref_time(10_044_265 as u64).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		Weight::from_ref_time(15_389_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		Weight::from_ref_time(27_609_000 as u64)
			// Standard Error: 16_042
			.saturating_add(Weight::from_ref_time(9_687_083 as u64).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		Weight::from_ref_time(31_466_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		Weight::from_ref_time(26_067_000 as u64)
			// Standard Error: 5_013
			.saturating_add(Weight::from_ref_time(9_041_847 as u64).saturating_mul(c as u64))
	}
}
