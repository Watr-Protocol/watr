#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	BoundedVec,
};
use pallet_did::types::{
	ServiceInfo,
	ServiceType::{self},
};
use pallet_evm::{
	AddressMapping, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
};
use precompile_utils::{revert, succeed, EvmDataWriter, RuntimeHelper};
use sp_core::H256;
use sp_std::marker::PhantomData;

use precompile_utils::{Address, Bytes, EvmResult, PrecompileHandleExt};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	CreateDID = "createDID(address,address,uint8[],bytes[])",
	CreateDIDOptional = "createDID(address,address,address,uint8[],bytes)",
	RemoveDID = "removeDID(address)",
	AddDIDServices = "addDidServices(address,uint8[],bytes[])",
	RemoveDIDServices = "removeDidServices(address,bytes32[])",
	IssueCredentials = "issueCredentials(address,address,bytes32,bytes)",
	RevokeCredentials = "revokeCredentials(address,bytes32)",
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
	<R as frame_system::Config>::Hash: From<H256>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;
		match selector {
			Action::CreateDID | Action::CreateDIDOptional => Self::create_did(handle, selector),
			Action::RemoveDID => Self::remove_did(handle),
			Action::AddDIDServices => Self::add_did_services(handle),
			Action::RemoveDIDServices => Self::remove_did_services(handle),
			Action::IssueCredentials => Self::issue_credentials(handle),
			Action::RevokeCredentials => Self::revoke_credentials(handle),
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
	<R as frame_system::Config>::Hash: From<H256>,
{
	fn create_did(
		handle: &mut impl PrecompileHandle,
		action: Action,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let (controller_raw, authentication, attestation_method, services_types, services_data) =
			match &action {
				Action::CreateDID => {
					input.expect_arguments(3)?;
					(
						input.read::<Address>()?,
						input.read::<Address>()?,
						None,
						input.read::<Vec<u8>>()?,
						input.read::<Vec<Bytes>>()?,
					)
				},
				Action::CreateDIDOptional => {
					input.expect_arguments(4)?;
					(
						input.read::<Address>()?,
						input.read::<Address>()?,
						Some(input.read::<Address>()?.0.into()),
						input.read::<Vec<u8>>()?,
						input.read::<Vec<Bytes>>()?,
					)
				},
				_ => unreachable!(),
			};
		let services: BoundedVec<ServiceInfo<R>, R::MaxServices> =
			Self::parse_services(services_types, services_data)?;

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

	fn update_did(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		todo!()
	}

	fn remove_did(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let origin = R::AddressMapping::into_account_id(handle.context().caller);
		let address = R::AddressMapping::into_account_id(input.read::<Address>()?.into());
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::remove_did { did: address.into() },
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn add_did_services(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let origin = R::AddressMapping::into_account_id(handle.context().caller);
		let did = R::AddressMapping::into_account_id(input.read::<Address>()?.into());
		let service_types = input.read::<Vec<u8>>()?;
		let service_details = input.read::<Vec<Bytes>>()?;

		let services = Self::parse_services(service_types, service_details)?;
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::add_did_services { did: did.into(), services },
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn remove_did_services(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let origin = R::AddressMapping::into_account_id(handle.context().caller);
		let did = R::AddressMapping::into_account_id(input.read::<Address>()?.into());
		let service_details = input.read::<Vec<H256>>()?;
		let mut services: BoundedVec<<R as frame_system::Config>::Hash, R::MaxServices> =
			BoundedVec::with_bounded_capacity(service_details.len());
		for service in service_details.iter() {
			let endpoint: <R as frame_system::Config>::Hash =
				service.to_owned().try_into().map_err(|_| revert("Services string too long"))?;
			match services.try_push(endpoint) {
				Ok(_) => {},
				Err(_) => return Err(revert("failed to parse to service")),
			}
		}
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::remove_did_services { did: did.into(), services_keys: services },
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn issue_credentials(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		todo!()
	}

	fn revoke_credentials(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		todo!()
	}

	fn parse_services(
		service_types: Vec<u8>,
		services_details: Vec<Bytes>,
	) -> Result<BoundedVec<ServiceInfo<R>, R::MaxServices>, PrecompileFailure> {
		if service_types.len() != services_details.len() {
			return Err(revert("Mismatched service types and descriptions"))
		}
		let mut services: BoundedVec<ServiceInfo<R>, R::MaxServices> =
			BoundedVec::with_bounded_capacity(service_types.len());
		let s = service_types.iter();
		let mut d = services_details.iter();
		for service in s {
			if let Some(detail) = d.next() {
				let service_type: ServiceType = match service {
					&0u8 => ServiceType::VerifiableCredentialFileStorage,
					_ => ServiceType::default(),
				};
				let endpoint: BoundedVec<u8, R::MaxString> =
					detail.clone().0.try_into().map_err(|_| revert("Services string too long"))?;
				match services
					.try_push(ServiceInfo { type_id: service_type, service_endpoint: endpoint })
				{
					Ok(_) => {},
					Err(_) => return Err(revert("failed to parse to service")),
				}
			}
		}
		Ok(services)
	}
}
