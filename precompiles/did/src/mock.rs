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
use frame_support::sp_runtime::BuildStorage;
use frame_support::{
	construct_runtime, parameter_types, sp_io,
	sp_runtime::traits::{BlakeTwo256, ConstU128, IdentityLookup},
	traits::Everything,
	weights::Weight,
};
use pallet_did;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use serde::{Deserialize, Serialize};
use sp_core::{H160, H256};

use pallet_evm::{
	EnsureAddressNever, EnsureAddressRoot, IsPrecompileResult, PrecompileResult, PrecompileSet,
};
use scale_info::TypeInfo;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 19;
	pub const DidDeposit: u64 = 5;
	pub const MaxString: u8 = 100;
	pub const MaxCredentialsTypes: u8 = 5;
	pub const MaxCredentialTypeLength: u32 = 32;
	pub const MaxServices: u8 = 5;
	pub const MaxHash: u32 = 512;
}

pub type AccountId = TestAccount;
pub type Balance = u128;
pub type Block = frame_system::mocking::MockBlock<Test>;

pub const PRECOMPILE_ADDRESS: H160 = H160::repeat_byte(0xDD);

/// A simple account type.
#[derive(
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Clone,
	Encode,
	Decode,
	Debug,
	MaxEncodedLen,
	Serialize,
	Deserialize,
	derive_more::Display,
	TypeInfo,
)]
pub enum TestAccount {
	Alice,
	Bob,
	Charlie,
	Bogus,
	Precompile,
}

impl Default for TestAccount {
	fn default() -> Self {
		Self::Alice
	}
}

impl AddressMapping<TestAccount> for TestAccount {
	fn into_account_id(h160_account: H160) -> TestAccount {
		match h160_account {
			a if a == H160::repeat_byte(0xAA) => Self::Alice,
			a if a == H160::repeat_byte(0xBB) => Self::Bob,
			a if a == H160::repeat_byte(0xCC) => Self::Charlie,
			a if a == PRECOMPILE_ADDRESS => Self::Precompile,
			_ => Self::Bogus,
		}
	}
}

impl From<H160> for TestAccount {
	fn from(x: H160) -> TestAccount {
		TestAccount::into_account_id(x)
	}
}

impl From<TestAccount> for H160 {
	fn from(value: TestAccount) -> H160 {
		match value {
			TestAccount::Alice => H160::repeat_byte(0xAA),
			TestAccount::Bob => H160::repeat_byte(0xBB),
			TestAccount::Charlie => H160::repeat_byte(0xCC),
			TestAccount::Precompile => PRECOMPILE_ADDRESS,
			TestAccount::Bogus => Default::default(),
		}
	}
}

impl From<TestAccount> for H256 {
	fn from(x: TestAccount) -> H256 {
		let x: H160 = x.into();
		x.into()
	}
}

impl From<TestAccount> for [u8; 32] {
	fn from(value: TestAccount) -> [u8; 32] {
		match value {
			TestAccount::Alice => [0xAA; 32],
			TestAccount::Bob => [0xBB; 32],
			TestAccount::Charlie => [0xCC; 32],
			_ => Default::default(),
		}
	}
}

impl From<TestAccount> for RuntimeOrigin {
	fn from(value: TestAccount) -> Self {
		Some(value).into()
	}
}

impl frame_system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u32;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_did::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DidIdentifier = TestAccount;
	type AuthenticationAddress = H160;
	type AssertionAddress = H160;
	type DidDeposit = DidDeposit;
	type MaxServices = MaxServices;
	type MaxString = MaxString;
	type MaxHash = MaxHash;
	type MaxCredentialsTypes = MaxCredentialsTypes;
	type MaxCredentialTypeLength = MaxCredentialTypeLength;
	type GovernanceOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = u128;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
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
	pub const PrecompilesValue: TestPrecompileSet<Test> =
	TestPrecompileSet(PhantomData);
	pub const WeightPerGas: Weight = Weight::from_parts(1, 0);
}

impl pallet_evm::Config for Test {
	type FeeCalculator = ();
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = EnsureAddressNever<AccountId>;
	type AddressMapping = AccountId;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = TestPrecompileSet<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ();
	type BlockGasLimit = ();
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = ();
	type GasLimitPovSizeRatio = ();
	type Timestamp = Timestamp;
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

#[derive(Debug, Clone, Copy)]
pub struct TestPrecompileSet<R>(PhantomData<R>);

impl<R> PrecompileSet for TestPrecompileSet<R>
where
	R: pallet_evm::Config + pallet_did::Config + frame_system::Config,
	<R as frame_system::pallet::Config>::RuntimeCall:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<R as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin:
		From<Option<R::AccountId>>,
	<R as frame_system::Config>::RuntimeCall: From<pallet_did::Call<R>>,
	<R as pallet_did::Config>::AuthenticationAddress: From<Address>,
	<R as frame_system::Config>::Hash: From<H256>,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			a if a == PRECOMPILE_ADDRESS => Some(WatrDIDPrecompile::<R>::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _remaining_gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer { is_precompile: address == PRECOMPILE_ADDRESS, extra_cost: 0 }
	}
}

construct_runtime!(
	pub struct Test {
		System: frame_system,
		Evm: pallet_evm,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
		DID: pallet_did,
	}
);

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = <frame_system::GenesisConfig<Test> as BuildStorage>::build_storage(
		&frame_system::GenesisConfig::default(),
	)
	.unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(TestAccount::Alice, 100),
			(TestAccount::Bob, 100),
			(TestAccount::Charlie, 100),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
