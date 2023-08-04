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

//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `PAR03651`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("devnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/watr-node
// benchmark
// pallet
// --chain=devnet-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_proxy
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./runtime/devnet/src/weights/pallet_proxy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		Weight::from_parts(16_000_000u64, 0)
			// Standard Error: 1_679
			.saturating_add(Weight::from_parts(62_536u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		Weight::from_parts(32_000_000u64, 0)
			// Standard Error: 2_798
			.saturating_add(Weight::from_parts(197_053u64, 0).saturating_mul(a as u64))
			// Standard Error: 2_787
			.saturating_add(Weight::from_parts(32_718u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3u64))
			.saturating_add(T::DbWeight::get().writes(2u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		Weight::from_parts(23_000_000u64, 0)
			// Standard Error: 1_739
			.saturating_add(Weight::from_parts(158_044u64, 0).saturating_mul(a as u64))
			// Standard Error: 1_732
			.saturating_add(Weight::from_parts(26_141u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2u64))
			.saturating_add(T::DbWeight::get().writes(2u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		Weight::from_parts(24_000_000u64, 0)
			// Standard Error: 1_764
			.saturating_add(Weight::from_parts(133_349u64, 0).saturating_mul(a as u64))
			// Standard Error: 1_757
			.saturating_add(Weight::from_parts(22_759u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2u64))
			.saturating_add(T::DbWeight::get().writes(2u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		Weight::from_parts(29_000_000u64, 0)
			// Standard Error: 2_794
			.saturating_add(Weight::from_parts(202_177u64, 0).saturating_mul(a as u64))
			// Standard Error: 2_783
			.saturating_add(Weight::from_parts(33_471u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3u64))
			.saturating_add(T::DbWeight::get().writes(2u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		Weight::from_parts(25_000_000u64, 0)
			// Standard Error: 2_970
			.saturating_add(Weight::from_parts(142_211u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		Weight::from_parts(25_000_000u64, 0)
			// Standard Error: 2_445
			.saturating_add(Weight::from_parts(115_634u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		Weight::from_parts(21_000_000u64, 0)
			// Standard Error: 2_769
			.saturating_add(Weight::from_parts(123_291u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		Weight::from_parts(28_000_000u64, 0)
			// Standard Error: 8_594
			.saturating_add(Weight::from_parts(224_087u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		Weight::from_parts(22_000_000u64, 0)
			// Standard Error: 5_885
			.saturating_add(Weight::from_parts(109_630u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
}
