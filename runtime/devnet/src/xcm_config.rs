use super::{
	weights::ExtrinsicBaseWeight, AccountId, AssetId, Assets, Authorship, Balance, Balances,
	ParachainInfo, ParachainSystem, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
	WeightToFee, XcmpQueue, GIGAWEI, MEGAWEI
};
use core::marker::PhantomData;
use frame_support::{
	log, match_types, parameter_types,
	traits::{Contains, Everything, Get, Nothing, PalletInfoAccess},
	weights::constants::WEIGHT_PER_SECOND,
};
use sp_std::{borrow::Borrow, result};

use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AsPrefixedGeneralIndex, ConvertedConcreteAssetId,
	CurrencyAdapter, EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds, FungiblesAdapter,
	IsConcrete, LocationInverter, NativeAsset, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
};
use xcm_executor::{
	traits::{Convert, FilterAssetLocation, JustTry},
	XcmExecutor,
};

use cumulus_primitives_utility::{ParentAsUmp, XcmFeesTo32ByteAccount};

use parachains_common::{
	impls::DealWithFees,
	xcm_config::{DenyReserveTransferToRelayChain, DenyThenTry},
};

use frame_system::EnsureRoot;

parameter_types! {
	pub const USDT: (u128, u128) = (1984, 1984); // (Reserve AssetId, Local AssetId)
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	pub SelfReserve: MultiLocation = MultiLocation { parents:0, interior: Here };
	pub StatemintLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(1000)));
	pub StatemintAssetsPalletLocation: MultiLocation =
		MultiLocation::new(1, X2(Parachain(1000), PalletInstance(50)));
	pub const RelayNetwork: NetworkId = NetworkId::Polkadot;
	pub SelfAssetsPalletLocation: MultiLocation = PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	pub CheckingAccount: AccountId = PolkadotXcm::check_account();
	pub USDTperSecond: (xcm::v1::AssetId, u128) = (
		MultiLocation::new(1, X3(Parachain(1000), PalletInstance(50), GeneralIndex(USDT::get().1))).into(),
		default_fee_per_second() * 10
	);
	pub XcmAssetFeesReceiver: Option<AccountId> = Authorship::author();
}

pub fn base_tx_fee() -> Balance {
	// GIGAWEI
	MEGAWEI
}

pub fn default_fee_per_second() -> u128 {
	let base_weight = Balance::from(ExtrinsicBaseWeight::get().ref_time());
	let base_tx_per_second = (WEIGHT_PER_SECOND.ref_time() as u128) / base_weight;
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
	CheckingAccount,
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
	// We dont want to allow teleporting assets
	Nothing,
	// The account to use for tracking teleports.
	CheckingAccount,
>;

pub struct AsForeignToLocal<Prefix, Asset, AssetId, ConvertAssetId>(
	PhantomData<(Prefix, Asset, AssetId, ConvertAssetId)>,
);
impl<
		Prefix: Get<MultiLocation>,
		Asset: Get<(u128, u128)>,
		AssetId: Clone,
		ConvertAssetId: Convert<u128, AssetId>,
	> Convert<MultiLocation, AssetId> for AsForeignToLocal<Prefix, Asset, AssetId, ConvertAssetId>
{
	fn convert_ref(id: impl Borrow<MultiLocation>) -> result::Result<AssetId, ()> {
		let prefix = Prefix::get();
		let asset = Asset::get();
		let id = id.borrow();

		match id.match_and_split(&prefix) {
			Some(&GeneralIndex(asset_id)) =>
				if asset_id == asset.0 {
					ConvertAssetId::convert_ref(&asset.1)
				} else {
					return Err(())
				},
			_ => Err(()),
		}
	}

	fn reverse_ref(what: impl Borrow<AssetId>) -> result::Result<MultiLocation, ()> {
		let mut location = Prefix::get();
		let id = ConvertAssetId::reverse_ref(what)?;
		let asset = Asset::get();

		if id == asset.1 {
			log::trace!(
				target: "xcm::execute_xcm_in_credit",
				"location: {:?}",
				location
			);
			location.push_interior(Junction::GeneralIndex(asset.0)).map_err(|_| ())?;
		} else {
			return Err(())
		}

		Ok(location)
	}
}

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
	pub UnitWeightCost: u64 = 1_000_000_000;
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
		// AllowUnpaidExecutionFrom<Statemint>,
		// Expected responses are OK.
		AllowKnownQueryResponses<PolkadotXcm>,
		// Subscriptions for version tracking are OK.
		AllowSubscriptionsFrom<Everything>,
	),
>;

//- From PR https://github.com/paritytech/cumulus/pull/936
fn matches_prefix(prefix: &MultiLocation, loc: &MultiLocation) -> bool {
	prefix.parent_count() == loc.parent_count() &&
		loc.len() >= prefix.len() &&
		prefix
			.interior()
			.iter()
			.zip(loc.interior().iter())
			.all(|(prefix_junction, junction)| prefix_junction == junction)
}
pub struct ReserveAssetsFrom<T>(PhantomData<T>);
impl<T: Get<MultiLocation>> FilterAssetLocation for ReserveAssetsFrom<T> {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		let prefix = T::get();
		log::trace!(target: "xcm::AssetsFrom", "prefix: {:?}, origin: {:?}", prefix, origin);
		&prefix == origin &&
			match asset {
				MultiAsset { id: xcm::latest::AssetId::Concrete(asset_loc), fun: Fungible(_a) } =>
					matches_prefix(&prefix, asset_loc),
				_ => false,
			}
	}
}

pub type Reserves = (NativeAsset, ReserveAssetsFrom<StatemintLocation>);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = Reserves;
	type IsTeleporter = (); // Teleporting is disabled.
	type LocationInverter = LocationInverter<Ancestry>;
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
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	ParentAsUmp<ParachainSystem, PolkadotXcm>,
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
	type LocationInverter = LocationInverter<Ancestry>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

pub struct AllowOnlySendToReservePerAsset<SelfLocation, ReserveAssetPallet, Assets>(
	PhantomData<(SelfLocation, ReserveAssetPallet, Assets)>,
);
impl<
		SelfLocation: Get<MultiLocation>,
		ReserveAssetPallet: Get<MultiLocation>,
		Assets: Get<(u128, u128)>,
	> Contains<(MultiLocation, Xcm<RuntimeCall>)>
	for AllowOnlySendToReservePerAsset<SelfLocation, ReserveAssetPallet, Assets>
{
	fn contains(t: &(MultiLocation, Xcm<RuntimeCall>)) -> bool {
		let message = &t.1;
		let assets = Assets::get();
		let reserve_asset_id = assets.0;

		let mut reserve_pallet_location = ReserveAssetPallet::get();
		let mut reserve_location = reserve_pallet_location.clone();
		reserve_location.take_last();

		if reserve_pallet_location.append_with(X1(GeneralIndex(reserve_asset_id))).is_err() { return false };
		let reserve_asset_location = reserve_pallet_location.clone();
		let self_location = SelfLocation::get();
		if reserve_pallet_location.reanchor(&reserve_location, &self_location).is_err() { return false };
		let reserve_asset_location_as_local = reserve_pallet_location.clone();
		let mut withdraw_amount = 0;
		let mut buy_amount = 1;

		let Xcm(inner_message) = message;

		if inner_message.len() == 2 {
			let withdraw_is_correct = match &inner_message[0] {
				WithdrawAsset(assets) => {
					let asset_is_correct = if let Some(asset) = assets.get(0) {
						match asset {
							MultiAsset { id: Concrete(reserve_asset), fun: Fungible(amount) } => {
								withdraw_amount = *amount;
								reserve_asset == &reserve_asset_location
							},
							_ => false,
						}
					} else {
						false
					};
					asset_is_correct && assets.len() == 1
				},
				_ => false,
			};

			let initiate_reserve_withdraw_is_correct = match &inner_message[1] {
				InitiateReserveWithdraw { assets: Wild(all), reserve, xcm: Xcm(inner_xcm) } =>
					if inner_xcm.len() == 2 && all == &All && reserve == &reserve_location {
						let buy_execution_is_correct = match &inner_xcm[0] {
							BuyExecution {
								fees:
									MultiAsset { id: Concrete(reserve_asset), fun: Fungible(amount) },
								weight_limit: Unlimited,
							} => {
								buy_amount = *amount;
								reserve_asset == &reserve_asset_location_as_local
							},
							_ => false,
						};
						let deposit_assset_is_correct = matches!(
							&inner_xcm[1],
							DepositAsset {
								assets: Wild(All),
								max_assets: 1,
								beneficiary: MultiLocation {
									parents: 0,
									interior: X1(AccountId32 { network: Any, id: _ })
								}
							}
						);
						buy_execution_is_correct && deposit_assset_is_correct
					} else {
						false
					},
				_ => false,
			};

			return withdraw_is_correct &&
				initiate_reserve_withdraw_is_correct &&
				(withdraw_amount >= buy_amount)
		}
		false
	}
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}
