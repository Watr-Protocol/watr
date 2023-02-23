use super::*;
use codec::{Decode, Encode, MaxEncodedLen, WrapperTypeEncode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug};
use frame_support::pallet_prelude::{CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};

#[derive(Clone, PartialEq, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AuthenticationMethod<T: Config> {
	pub controller: T::AuthenticationAddress,
}

#[derive(Clone, PartialEq, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct AssertionMethod<T: Config> {
	pub controller: T::AssertionAddress,
}

#[derive(Clone, Default, Decode, Encode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
enum ServiceType {
	#[default]
	VerifiableCredentialFileStorage
}

#[derive(CloneNoBound, PartialEqNoBound, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Service<T: Config> {
	type_id: ServiceType,
	pub service_endpoint: BoundedVec<u8, T::MaxString>,
}

#[derive(CloneNoBound, PartialEqNoBound, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Document<T: Config> {
	pub controller: DidIdentifierOf<T>,
	pub authentication: AuthenticationMethod<T>,
	pub assertion_method: Option<AssertionMethod<T>>,
	pub services: Option<BoundedVec<KeyIdOf<T>, T::MaxServices>>,
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
	pub status: IssuerStatus
}
