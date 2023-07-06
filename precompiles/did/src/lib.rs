#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	BoundedVec,
};
use pallet_did::types::{ServiceInfo, ServiceType::VerifiableCredentialFileStorage};
use pallet_evm::{AddressMapping, Precompile, PrecompileHandle, PrecompileOutput};
use precompile_utils::{error, succeed, EvmDataWriter, RuntimeHelper};
use sp_std::marker::PhantomData;

use precompile_utils::{Address, Bytes, EvmResult, PrecompileHandleExt};

#[cfg(test)]
mod mock;

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	CreateDID = "create_did(address, address, bytes)",
	CreateDIDOptional = "create_did(address, address, address, bytes)",
}

pub struct WatrDIDPrecompile<R>(PhantomData<R>);

impl<R> Precompile for WatrDIDPrecompile<R>
where
	R: pallet_evm::Config + pallet_did::Config + frame_system::Config,
	<R as frame_system::pallet::Config>::RuntimeCall:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<R as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin: From<R::AccountId>,
	<R as frame_system::Config>::RuntimeCall: From<pallet_did::Call<R>>,
	<R as pallet_did::Config>::AuthenticationAddress: From<Address>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;
		match selector {
			Action::CreateDID | Action::CreateDIDOptional => Self::create_did(handle, selector),
		}
	}
}

impl<R> WatrDIDPrecompile<R>
where
	R: pallet_evm::Config + pallet_did::Config,
	<R as frame_system::pallet::Config>::RuntimeCall:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<R as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin: From<R::AccountId>,
	<R as frame_system::Config>::RuntimeCall: From<pallet_did::Call<R>>,
	<R as pallet_did::Config>::AuthenticationAddress: From<Address>,
{
	fn create_did(
		handle: &mut impl PrecompileHandle,
		action: Action,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let (controller_raw, authentication, attestation_method, services_raw) = match &action {
			Action::CreateDID => {
				input.expect_arguments(3)?;
				(input.read::<Address>()?, input.read::<Address>()?, None, input.read::<Bytes>()?)
			},
			Action::CreateDIDOptional => {
				input.expect_arguments(4)?;
				(
					input.read::<Address>()?,
					input.read::<Address>()?,
					Some(input.read::<Address>()?.0.into()),
					input.read::<Bytes>()?,
				)
			},
		};
		let endpoint: BoundedVec<u8, R::MaxString> =
			services_raw.0.try_into().map_err(|_| error("Services string too long"))?;
		let services: BoundedVec<ServiceInfo<R>, R::MaxServices> = vec![ServiceInfo {
			type_id: VerifiableCredentialFileStorage,
			service_endpoint: endpoint,
		}]
		.try_into()
		.map_err(|_| error("Too many services"))?;

		let origin = R::AddressMapping::into_account_id(handle.context().caller);
		let controller = R::AddressMapping::into_account_id(controller_raw.into());
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::create_did {
				controller: controller.into(),
				authentication: authentication.into(),
				assertion: attestation_method,
				services,
			},
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}
}

#[cfg(test)]
mod tests {}
