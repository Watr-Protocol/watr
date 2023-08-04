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
pub(crate) use crate as pallet_did;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64},
};
use sp_core::{H160, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub struct Test{
		System: frame_system::{Pallet, Call, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		DID: pallet_did,
	}
);

impl frame_system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type FreezeIdentifier = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type MaxHolds = ();
	type MaxFreezes = ();
}

parameter_types! {
	pub const MaxString: u8 = 100;
	pub const MaxCredentialsTypes: u8 = 50;
	pub const MaxCredentialTypeLength: u32 = 32;
	pub const MaxServices: u8 = 10;
	pub const MaxHash: u32 = 512;
	pub const DidDeposit: u64 = 5;
}

impl pallet_did::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DidIdentifier = u64;
	type AuthenticationAddress = H160;
	type AssertionAddress = H160;
	type DidDeposit = DidDeposit;
	type MaxServices = MaxServices;
	type MaxString = MaxString;
	type MaxHash = MaxHash;
	type MaxCredentialsTypes = MaxCredentialsTypes;
	type MaxCredentialTypeLength = MaxCredentialTypeLength;
	type GovernanceOrigin = frame_system::EnsureRoot<u64>;
	type WeightInfo = ();
}

pub(crate) const ALICE: u64 = 1;
pub(crate) const BOB: u64 = 2;
pub(crate) const ACCOUNT_00: u64 = 0;
pub(crate) const ACCOUNT_01: u64 = 1;
pub(crate) const ACCOUNT_02: u64 = 2;
pub(crate) const ACCOUNT_03: u64 = 3;
pub(crate) const ACCOUNT_04: u64 = 4;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = RuntimeGenesisConfig {
		balances: pallet_balances::GenesisConfig::<Test> {
			balances: vec![(0, 2), (1, 10), (2, 20), (3, 30), (4, 40), (5, 50)],
		},
	}
	.build_storage()
	.unwrap()
	.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
