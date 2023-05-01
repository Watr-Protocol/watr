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

// This file was originally forked from Substrate Parachain Template
// which is generated directly to the upstream Parachain Template in Cumulus
// https://github.com/paritytech/cumulus/tree/master/parachain-template

use super::{
	weights, weights::ExtrinsicBaseWeight, AccountId, AllPalletsWithSystem, AssetId, Assets,
	Authorship, Balance, Balances, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime,
	RuntimeCall, RuntimeEvent, RuntimeOrigin, WeightToFee, XcmpQueue, KILOWEI,
};
use frame_support::{
	match_types, parameter_types,
	traits::{ConstU32, Everything, Nothing, PalletInfoAccess},
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};

use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use xcm::{latest::prelude::*, prelude::AssetId as XcmAssetId};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AsPrefixedGeneralIndex, ConvertedConcreteAssetId,
	CurrencyAdapter, EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds, FungiblesAdapter,
	IsConcrete, NativeAsset, NoChecking, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
};
use xcm_executor::{traits::JustTry, XcmExecutor};

use cumulus_primitives_utility::{ParentAsUmp, XcmFeesTo32ByteAccount};

use watr_common::{
	impls::DealWithFees,
	xcm_config::{AllowOnlySendToReservePerAsset, AsForeignToLocal, ConcreteNativeAssetFrom},
	xcm_config_parachains::{DenyReserveTransferToRelayChain, DenyThenTry},
};

use frame_system::EnsureRoot;

parameter_types! {
	pub const USDT: (u128, u128) = (1984, 1984); // (Reserve AssetId, Local AssetId)
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	// pub SelfReserve: MultiLocation = MultiLocation { parents:0, interior: Here };
	pub SelfReserve: Junctions = Junctions::Here;
	pub StatemintLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(1000)));
	pub StatemintAssetsPalletLocation: MultiLocation =
		MultiLocation::new(1, X2(Parachain(1000), PalletInstance(50)));
	pub const RelayNetwork: NetworkId = NetworkId::Polkadot;
	pub SelfAssetsPalletLocation: MultiLocation = PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	pub CheckingAccount: AccountId = PolkadotXcm::check_account();
	pub USDTperSecond: (XcmAssetId, u128, u128) = (
		MultiLocation::new(1, X3(Parachain(1000), PalletInstance(50), GeneralIndex(USDT::get().1))).into(),
		default_fee_per_second() * 10,
		0
	);
	pub UniversalLocation: InteriorMultiLocation =
		X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
	pub XcmAssetFeesReceiver: Option<AccountId> = Authorship::author();
	pub const MaxAssetsIntoHolding: u32 = 64;
}

pub fn base_tx_fee() -> Balance {
	KILOWEI / 10
}

pub fn default_fee_per_second() -> u128 {
	let base_weight = Balance::from(ExtrinsicBaseWeight::get().ref_time());
	let base_tx_per_second = (WEIGHT_REF_TIME_PER_SECOND as u128) / base_weight;
	base_tx_per_second * base_tx_fee()
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Means for transacting assets on this chain.
pub type CurrencyTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<SelfReserve>,
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

/// Means for transacting local assets besides the native currency on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	Assets,
	// Use this currency when it is a fungible asset matching the given location or name:
	(
		ConvertedConcreteAssetId<
			AssetId,
			Balance,
			AsPrefixedGeneralIndex<SelfAssetsPalletLocation, AssetId, JustTry>, // For local references
			JustTry,
		>,
		ConvertedConcreteAssetId<
			AssetId,
			Balance,
			AsForeignToLocal<StatemintAssetsPalletLocation, USDT, AssetId, JustTry>, // For remote references (foreign)
			JustTry,
		>,
	),
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We do not want to allow teleporing of assets
	NoChecking,
	// The account to use for tracking teleports.
	CheckingAccount,
>;

/// Means for transacting assets on this chain.
pub type AssetTransactors = (CurrencyTransactor, FungiblesTransactor);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
	pub const MaxInstructions: u32 = 100;
}

match_types! {
	pub type Statemint: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: X1(Parachain(1000)) }
	};
}

pub type Barrier = DenyThenTry<
	DenyReserveTransferToRelayChain,
	(
		TakeWeightCredit,
		AllowTopLevelPaidExecutionFrom<Everything>,
		// Expected responses are OK.
		AllowKnownQueryResponses<PolkadotXcm>,
		// Subscriptions for version tracking are OK.
		AllowSubscriptionsFrom<Everything>,
	),
>;

pub type Reserves = (NativeAsset, ConcreteNativeAssetFrom<StatemintLocation>);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = Reserves;
	// Teleporting is disabled.
	type IsTeleporter = ();
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = (
		FixedRateOfFungible<
			USDTperSecond,
			XcmFeesTo32ByteAccount<FungiblesTransactor, AccountId, XcmAssetFeesReceiver>,
		>,
		UsingComponents<WeightToFee, SelfReserve, AccountId, Balances, DealWithFees<Runtime>>,
	);
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

pub type XcmExecuteFilter =
	AllowOnlySendToReservePerAsset<SelfReserve, StatemintAssetsPalletLocation, USDT>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	// We want to disallow users sending (arbitrary) XCMs from this chain. (only Root)
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, ()>;
	type XcmRouter = XcmRouter;
	// We support local origins dispatching XCM executions in principle...
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	// ... but disallow arbitrary XCM messages execution.
	// As a result only reserve transfers back to the reserve of specific assets are allowed.
	type XcmExecuteFilter = XcmExecuteFilter;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Nothing;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type UniversalLocation = UniversalLocation;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = PolkadotXcm;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = weights::cumulus_pallet_xcmp_queue::WeightInfo<Runtime>;
	type PriceForSiblingDelivery = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}
