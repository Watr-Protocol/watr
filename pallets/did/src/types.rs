use super::*;
use codec::{Decode, Encode, MaxEncodedLen, WrapperTypeEncode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug};
use frame_support::pallet_prelude::RuntimeDebugNoBound;

#[derive(Clone, Default, Decode, Encode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AuthenticationMethod<T: Config> {
	controller: T::AuthenticationAddress,
}

#[derive(Clone, Default, Decode, Encode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AssertionMethod<T: Config> {
	controller: T::AssertionAddress,
}

#[derive(Clone, Default, Decode, Encode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
enum ServiceType {
	#[default]
	VerifiableCredentialFileStorage
}

#[derive(Clone, Decode, Default, Encode, PartialEq, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Service<T: Config> {
	type_id: ServiceType,
	service_endpoint: BoundedVec<u8, T::MaxString>,
}

#[derive(Clone, Decode, Default, Encode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Document<T: Config> {
	controller: DidIdentifierOf<T>,
	authentication: AuthenticationMethod<T>,
	assertion_method: Option<AssertionMethod<T>>,
	services: Option<BoundedVec<KeyIdOf<T>, T::MaxServices>>,
}

#[derive(Clone, Decode, Default, Encode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub enum IssuerStatus {
	#[default]
	Active,
	Revoked
}

#[derive(Clone, Decode, Default, Encode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct IssuerInfo {
	status: IssuerStatus
}
