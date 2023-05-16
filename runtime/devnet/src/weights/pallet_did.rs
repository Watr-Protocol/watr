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

//! Autogenerated weights for `pallet_did`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-10-2-102-127`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("devnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/watr-node
// benchmark
// pallet
// --chain=devnet-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_did
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./runtime/devnet/src/weights/pallet_did.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_did`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_did::WeightInfo for WeightInfo<T> {
	// Storage: DID Did (r:1 w:1)
	// Storage: DID Issuers (r:1 w:0)
	// Storage: DID Services (r:1 w:1)
	/// The range of component `m` is `[0, 10]`.
	fn create_did(m: u32, ) -> Weight {
		Weight::from_ref_time(67_964_000 as u64)
			// Standard Error: 34_395
			.saturating_add(Weight::from_ref_time(7_352_037 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(m as u64)))
	}
	// Storage: DID Did (r:1 w:1)
	fn update_did() -> Weight {
		Weight::from_ref_time(46_022_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DID Did (r:1 w:1)
	// Storage: DID Issuers (r:1 w:0)
	// Storage: DID Services (r:1 w:1)
	/// The range of component `m` is `[0, 10]`.
	fn remove_did(m: u32, ) -> Weight {
		Weight::from_ref_time(69_123_000 as u64)
			// Standard Error: 38_992
			.saturating_add(Weight::from_ref_time(7_942_168 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(m as u64)))
	}
	// Storage: DID Did (r:1 w:1)
	// Storage: DID Services (r:1 w:1)
	/// The range of component `m` is `[0, 10]`.
	fn add_did_services(m: u32, ) -> Weight {
		Weight::from_ref_time(47_942_000 as u64)
			// Standard Error: 30_743
			.saturating_add(Weight::from_ref_time(6_967_329 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(m as u64)))
	}
	// Storage: DID Did (r:1 w:1)
	// Storage: DID Services (r:1 w:1)
	/// The range of component `m` is `[0, 10]`.
	fn remove_did_services(m: u32, ) -> Weight {
		Weight::from_ref_time(43_431_000 as u64)
			// Standard Error: 36_597
			.saturating_add(Weight::from_ref_time(7_833_852 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(m as u64)))
	}
	// Storage: DID Did (r:2 w:0)
	// Storage: DID Issuers (r:1 w:0)
	// Storage: DID CredentialsTypes (r:1 w:0)
	// Storage: DID IssuedCredentials (r:1 w:1)
	/// The range of component `c` is `[0, 50]`.
	fn issue_credentials(c: u32, ) -> Weight {
		Weight::from_ref_time(57_730_000 as u64)
			// Standard Error: 98_950
			.saturating_add(Weight::from_ref_time(9_786_295 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: DID Did (r:1 w:0)
	// Storage: DID IssuedCredentials (r:1 w:1)
	/// The range of component `c` is `[0, 50]`.
	fn revoke_credentials(c: u32, ) -> Weight {
		Weight::from_ref_time(43_708_000 as u64)
			// Standard Error: 290_119
			.saturating_add(Weight::from_ref_time(14_030_066 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: DID Did (r:1 w:0)
	// Storage: DID Issuers (r:1 w:1)
	fn add_issuer() -> Weight {
		Weight::from_ref_time(49_897_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DID Issuers (r:1 w:1)
	fn revoke_issuer() -> Weight {
		Weight::from_ref_time(45_042_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DID Issuers (r:1 w:1)
	fn reactivate_issuer() -> Weight {
		Weight::from_ref_time(45_100_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DID CredentialsTypes (r:1 w:1)
	/// The range of component `m` is `[0, 50]`.
	fn add_credentials_type(m: u32, ) -> Weight {
		Weight::from_ref_time(32_808_000 as u64)
			// Standard Error: 12_666
			.saturating_add(Weight::from_ref_time(1_000_865 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DID CredentialsTypes (r:1 w:1)
	/// The range of component `m` is `[0, 50]`.
	fn remove_credentials_type(m: u32, ) -> Weight {
		Weight::from_ref_time(34_872_000 as u64)
			// Standard Error: 13_147
			.saturating_add(Weight::from_ref_time(884_187 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
