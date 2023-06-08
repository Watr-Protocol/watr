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

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `PAR03610`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/watr-node
// benchmark
// pallet
// --chain=mainnet-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_assets
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./runtime/mainnet/src/weights/pallet_assets.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3687`
		// Minimum execution time: 27_000_000 picoseconds.
		Weight::from_parts(27_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128`
		//  Estimated: `3687`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3687`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1001 w:1000)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: System Account (r:1000 w:1000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `63 + c * (208 ±0)`
		//  Estimated: `3687 + c * (2603 ±0)`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(15_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			// Standard Error: 4_592
			.saturating_add(Weight::from_parts(11_185_924, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(c.into()))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1001 w:1000)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476 + a * (86 ±0)`
		//  Estimated: `3687 + a * (2635 ±0)`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			// Standard Error: 5_880
			.saturating_add(Weight::from_parts(15_167_838, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2635).saturating_mul(a.into()))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378`
		//  Estimated: `3687`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378`
		//  Estimated: `3687`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `450`
		//  Estimated: `3687`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `489`
		//  Estimated: `6168`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(42_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6168))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `489`
		//  Estimated: `6168`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6168))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `489`
		//  Estimated: `6168`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(42_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6168))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `450`
		//  Estimated: `3687`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `450`
		//  Estimated: `3687`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3687`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3687`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378`
		//  Estimated: `3687`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3687`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378`
		//  Estimated: `3687`
		// Minimum execution time: 27_000_000 picoseconds.
		Weight::from_parts(28_130_715, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			// Standard Error: 878
			.saturating_add(Weight::from_parts(996, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `507`
		//  Estimated: `3687`
		// Minimum execution time: 27_000_000 picoseconds.
		Weight::from_parts(28_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `3687`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_067_390, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `507`
		//  Estimated: `3687`
		// Minimum execution time: 27_000_000 picoseconds.
		Weight::from_parts(27_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3687`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(11_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3687`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `671`
		//  Estimated: `6168`
		// Minimum execution time: 60_000_000 picoseconds.
		Weight::from_parts(61_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6168))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509`
		//  Estimated: `3687`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(32_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509`
		//  Estimated: `3687`
		// Minimum execution time: 32_000_000 picoseconds.
		Weight::from_parts(33_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3687`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3687))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
