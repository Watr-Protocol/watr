use super::*;
use codec::{Decode, Encode, MaxEncodedLen, WrapperTypeEncode};
use scale_info::TypeInfo;

#[derive(Decode, Encode, PartialEq, TypeInfo, MaxEncodedLen)]
pub struct AuthenticationMethod<T: Config> {
	controller: T::AuthenticationMethod,
}

#[derive(Decode, Encode, PartialEq, TypeInfo, MaxEncodedLen)]
pub struct AssertionMethod<T: Config> {
	controller: T::AssertionMethod,
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
enum ServiceType {
	VerifiableCredentialFileStorage
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
pub struct Service<T: Config> {
	type_id: ServiceType,
	service_endpoint: BoundedVec<u8, T::MaxString>,
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
pub struct Document<T: Config> {
	controller: DidIdentifierOf<T>,
	authentication: AuthenticationMethod<T>,
	assertion_method: Option<AssertionMethod<T>>,
	services: Option<Vec<Service<T>>>,
}

#[derive(Clone, Decode, Encode, PartialEq, TypeInfo, MaxEncodedLen)]
pub enum IssuerStatus {
	Active,
	Revoked
}

impl Default for IssuerStatus {
	fn default() -> Self {
		Self::Active
	}
}

#[derive(Clone, Decode, Default, Encode, PartialEq, TypeInfo, MaxEncodedLen)]
pub struct IssuerInfo {
	status: IssuerStatus
}
