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

use core::marker::PhantomData;
use frame_support::{
	log,
	traits::{Contains, ContainsPair, Get},
};
use sp_std::{borrow::Borrow, result};

use xcm::latest::prelude::*;
use xcm_executor::traits::Convert;

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

/// Accepts an asset if it is a native asset from a particular `MultiLocation`.
pub struct ConcreteNativeAssetFrom<Location>(PhantomData<Location>);
impl<Location: Get<MultiLocation>> ContainsPair<MultiAsset, MultiLocation>
	for ConcreteNativeAssetFrom<Location>
{
	fn contains(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		let prefix = Location::get();
		log::trace!(target: "xcm::filter_asset_location", "prefix: {:?}, origin: {:?}", prefix, origin);
		&prefix == origin &&
			match asset {
				MultiAsset { id: xcm::latest::AssetId::Concrete(asset_loc), fun: Fungible(_a) } =>
					matches_prefix(&prefix, asset_loc),
				_ => false,
			}
	}
}

pub struct AllowOnlySendToReservePerAsset<SelfLocation, ReserveAssetPallet, Assets>(
	PhantomData<(SelfLocation, ReserveAssetPallet, Assets)>,
);
impl<
		SelfLocation: Get<Junctions>,
		ReserveAssetPallet: Get<MultiLocation>,
		Assets: Get<(u128, u128)>,
		RuntimeCall,
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

		if reserve_pallet_location.append_with(X1(GeneralIndex(reserve_asset_id))).is_err() {
			return false
		};
		let reserve_asset_location = reserve_pallet_location.clone();
		let self_location = SelfLocation::get();
		if reserve_pallet_location.reanchor(&reserve_location, self_location).is_err() {
			return false
		};
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
						let deposit_asset_is_correct = matches!(
							&inner_xcm[1],
							DepositAsset {
								assets: Wild(All),
								// max_assets: 1,
								beneficiary: MultiLocation {
									parents: 0,
									interior: X1(AccountId32 { network: _, id: _ })
								}
							}
						);
						buy_execution_is_correct && deposit_asset_is_correct
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
