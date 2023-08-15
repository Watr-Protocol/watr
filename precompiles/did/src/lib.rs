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
use sp_std::vec::Vec;

use precompile_utils::{Address, Bytes, EvmResult, PrecompileHandleExt};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	CreateDID = "createDid(address,address,(bool,address),(uint8,string)[])",
	UpdateDID =
		"updateDid(address,(bool,address),(bool,address),(bool,address),(bool,(uint8,string)[]))",
	RemoveDID = "removeDid(address)",
	AddDIDServices = "addDidServices(address,(uint8,string)[])",
	RemoveDIDServices = "removeDidServices(address,bytes[])",
	IssueCredentials = "issueCredentials(address,address,string[],bytes)",
	RevokeCredentials = "revokeCredentials(address,string[])",
}

pub struct WatrDIDPrecompile<R>(PhantomData<R>);

impl<R> Precompile for WatrDIDPrecompile<R>
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
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;
		match selector {
			Action::CreateDID => Self::create_did(handle),
			Action::UpdateDID => Self::update_did(handle),
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
	<<R as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin:
		From<Option<R::AccountId>>,
	<R as frame_system::Config>::RuntimeCall: From<pallet_did::Call<R>>,
	<R as pallet_did::Config>::AuthenticationAddress: From<Address>,
	<R as frame_system::Config>::Hash: From<H256>,
{
	fn create_did(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(4)?;
		let (controller_raw, authentication, maybe_attestation_method, raw_services) = (
			input.read::<Address>()?,
			input.read::<Address>()?,
			input.read::<(bool, Address)>()?,
			input.read::<Vec<(u8, Bytes)>>()?,
		);
		let attestation_method =
			maybe_attestation_method.0.then(|| maybe_attestation_method.1 .0.into());
		let services: BoundedVec<ServiceInfo<R>, R::MaxServices> =
			Self::parse_services(raw_services)?;
		let origin = Some(R::AddressMapping::into_account_id(handle.context().caller));
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
		let mut input = handle.read_input()?;
		input.expect_arguments(5)?;
		let (
			did_raw,
			maybe_controller_raw,
			maybe_authentication,
			maybe_attestation_method,
			maybe_raw_services,
		) = (
			input.read::<Address>()?,
			input.read::<(bool, Address)>()?,
			input.read::<(bool, Address)>()?,
			input.read::<(bool, Address)>()?,
			input.read::<(bool, Vec<(u8, Bytes)>)>()?,
		);
		let controller = maybe_controller_raw
			.0
			.then(|| R::AddressMapping::into_account_id(maybe_controller_raw.1.into()).into());
		let authentication = maybe_authentication.0.then(|| maybe_authentication.1.into());
		let attestation_method =
			maybe_attestation_method.0.then(|| maybe_attestation_method.1 .0.into());
		let services: Option<BoundedVec<ServiceInfo<R>, R::MaxServices>> = maybe_raw_services
			.0
			.then(|| Self::parse_services(maybe_raw_services.1))
			.transpose()?;

		let origin = Some(R::AddressMapping::into_account_id(handle.context().caller));
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::update_did {
				did: R::AddressMapping::into_account_id(did_raw.into()).into(),
				controller,
				authentication,
				assertion: attestation_method,
				services,
			},
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn remove_did(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let did_raw = input.read::<Address>()?;
		let origin = Some(R::AddressMapping::into_account_id(handle.context().caller));
		let did = R::AddressMapping::into_account_id(did_raw.into());
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::remove_did { did: did.into() },
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn add_did_services(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let origin = Some(R::AddressMapping::into_account_id(handle.context().caller));
		let did = R::AddressMapping::into_account_id(input.read::<Address>()?.into());
		let raw_services = input.read::<Vec<(u8, Bytes)>>()?;

		let services = Self::parse_services(raw_services)?;
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::add_did_services { did: did.into(), services },
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn remove_did_services(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let origin = Some(R::AddressMapping::into_account_id(handle.context().caller));
		let did = R::AddressMapping::into_account_id(input.read::<Address>()?.into());
		let service_details = input.read::<Vec<Bytes>>()?;
		let mut services: BoundedVec<<R as frame_system::Config>::Hash, R::MaxServices> =
			BoundedVec::with_bounded_capacity(service_details.len());
		for service in service_details.into_iter() {
			if service.0.len() != H256::len_bytes() {
				return Err(revert("Service length different than 32 bytes"));
			}
			let hash = H256::from_slice(service.0.as_slice());
			let endpoint = <R as frame_system::Config>::Hash::from(hash);
			services.try_push(endpoint).map_err(|_| revert("failed to parse to service"))?;
		}
		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::remove_did_services { did: did.into(), services_keys: services },
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn parse_credentials(
		raw_credentials: Vec<Bytes>,
	) -> EvmResult<BoundedVec<BoundedVec<u8, R::MaxCredentialTypeLength>, R::MaxCredentialsTypes>> {
		let mut credentials = BoundedVec::with_bounded_capacity(raw_credentials.len());
		for raw_credential in raw_credentials {
			raw_credential
				.as_str()
				.map_err(|_| revert("Not a valid UTF8 credential string"))?;
			let credential = BoundedVec::try_from(raw_credential.0)
				.map_err(|_| revert("Credential too long"))?;
			credentials
				.try_push(credential)
				.map_err(|_| revert("failed to parse to credential"))?;
		}
		Ok(credentials)
	}

	fn issue_credentials(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(4)?;
		let origin = Some(R::AddressMapping::into_account_id(handle.context().caller));
		let issuer_did = R::AddressMapping::into_account_id(input.read::<Address>()?.into()).into();
		let subject_did =
			R::AddressMapping::into_account_id(input.read::<Address>()?.into()).into();
		let raw_credentials = input.read::<Vec<Bytes>>()?;
		let credentials = Self::parse_credentials(raw_credentials)?;
		let raw_verifiable_credential_hash = input.read::<Bytes>()?;
		let verifiable_credential_hash: BoundedVec<u8, R::MaxHash> = raw_verifiable_credential_hash
			.0
			.try_into()
			.map_err(|_| revert("Verifiable credential hash too long"))?;

		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::issue_credentials {
				issuer_did,
				subject_did,
				credentials,
				verifiable_credential_hash,
			},
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn revoke_credentials(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(3)?;

		let origin = Some(R::AddressMapping::into_account_id(handle.context().caller));
		let issuer_did = R::AddressMapping::into_account_id(input.read::<Address>()?.into()).into();
		let subject_did =
			R::AddressMapping::into_account_id(input.read::<Address>()?.into()).into();
		let raw_credentials = input.read::<Vec<Bytes>>()?;
		let credentials = Self::parse_credentials(raw_credentials)?;

		RuntimeHelper::<R>::try_dispatch(
			handle,
			origin.into(),
			pallet_did::Call::<R>::revoke_credentials { issuer_did, subject_did, credentials },
		)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn parse_services(
		raw_services: Vec<(u8, Bytes)>,
	) -> Result<BoundedVec<ServiceInfo<R>, R::MaxServices>, PrecompileFailure> {
		let mut services: BoundedVec<ServiceInfo<R>, R::MaxServices> =
			BoundedVec::with_bounded_capacity(raw_services.len());
		let s = raw_services.iter();
		for service in s {
			let service_type: ServiceType = match service.0 {
				0u8 => ServiceType::VerifiableCredentialFileStorage,
				_ => ServiceType::default(),
			};
			service.1.as_str().map_err(|_| revert("Not a valid UTF8 service string"))?;
			let endpoint: BoundedVec<u8, R::MaxString> =
				service.1.clone().0.try_into().map_err(|_| revert("Services string too long"))?;
			services
				.try_push(ServiceInfo { type_id: service_type, service_endpoint: endpoint })
				.map_err(|_| revert("failed to parse to service"))?;
		}
		Ok(services)
	}
}
