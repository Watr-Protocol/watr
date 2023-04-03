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

// This file was originally copied from the The Astar Network EVM precompiles
// and used in terms of GPLv3.
// https://github.com/AstarNetwork/Astar/blob/master/runtime/astar/src/precompiles.rs

use pallet_evm::{
	AddressMapping, ExitRevert, Precompile, PrecompileFailure, PrecompileHandle, PrecompileResult,
	PrecompileSet,
};
use pallet_evm_precompile_assets_erc20::{AddressToAssetId, Erc20AssetsPrecompileSet};
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, Identity, Ripemd160, Sha256};
use precompile_utils::error;
use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::fungibles::roles::Inspect;

use crate::AssetId;

/// The asset precompile address prefix. Addresses that match against this prefix will be routed
/// to Erc20AssetsPrecompileSet
pub const ASSET_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[255u8; 4];

pub const TUSD_PRECOMPILE_ADDRESS: AssetId = 2010;

/// The PrecompileSet installed in the Astar runtime.
#[derive(Debug, Default, Clone, Copy)]
pub struct FrontierPrecompiles<R>(PhantomData<R>);

impl<R> FrontierPrecompiles<R> {
	pub fn new() -> Self {
		Self(Default::default())
	}

	/// Return all addresses that contain precompiles. This can be used to populate dummy code
	/// under the precompile.
	pub fn used_addresses() -> impl Iterator<Item = H160> {
		sp_std::vec![
			1, 2, 3, 4, 5, 6, 7, 8, 9, 1024, 1025, 1026, 1027, 20481, 20482, 20483, 20484
		]
		.into_iter()
		.map(hash)
	}
}

/// The following distribution has been decided for the precompiles
/// 0-1023: Ethereum Mainnet Precompiles
/// 1024-2047 Precompiles that are not in Ethereum Mainnet
impl<R> pallet_evm::PrecompileSet for FrontierPrecompiles<R>
where
	Erc20AssetsPrecompileSet<R>: PrecompileSet,
	<R as pallet_assets::Config>::AssetId: From<AssetId>,
	R: pallet_evm::Config
		+ pallet_assets::Config
		+ pallet_xcm::Config
		+ AddressToAssetId<<R as pallet_assets::Config>::AssetId>,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		let address = handle.code_address();
		if self.is_precompile(address) && address > hash(9) && handle.context().address != address {
			return Some(Err(PrecompileFailure::Revert {
				exit_status: ExitRevert::Reverted,
				output: b"cannot be called with DELEGATECALL or CALLCODE".to_vec(),
			}))
		}
		match address {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			a if a == hash(6) => Some(Bn128Add::execute(handle)),
			a if a == hash(7) => Some(Bn128Mul::execute(handle)),
			a if a == hash(8) => Some(Bn128Pairing::execute(handle)),
			a if a == hash(9) => Some(Blake2F::execute(handle)),
			// Non-Frontier specific nor Ethereum precompiles :
			// nor Ethereum precompiles :
			a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			// If the address matches asset prefix, the we route through the asset precompile set
			a if &a.to_fixed_bytes()[0..4] == ASSET_PRECOMPILE_ADDRESS_PREFIX => {
				// Get asset id to check if it is TUSD
				let asset_id = R::address_to_asset_id(a).unwrap_or(0.into());
				// If asset is TUSD, ensure origin is TUSD contract. May return error
				if asset_id == TUSD_PRECOMPILE_ADDRESS.into() {
					let owner = pallet_assets::Pallet::<R>::issuer(asset_id);
					let origin = R::AddressMapping::into_account_id(handle.context().caller);

					if Some(origin.clone()) != owner.clone() {
						return Some(Err(error("bad origin for tusd precompile")))
					}
				}

				Erc20AssetsPrecompileSet::<R>::new().execute(handle)
			},

			// Default
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		Self::used_addresses().any(|x| x == address) ||
			Erc20AssetsPrecompileSet::<R>::new().is_precompile(address)
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
