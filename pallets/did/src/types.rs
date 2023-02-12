use super::*;
use codec::{Decode, Encode, MaxEncodedLen, WrapperTypeEncode};
use scale_info::TypeInfo;

#[derive(Decode, Encode, PartialEq, TypeInfo, MaxEncodedLen)]
pub struct Authentication<T: Config> {
	controller: T::DidIdentifier,
}

#[derive(Decode, Encode, PartialEq, TypeInfo, MaxEncodedLen)]
pub struct AssertionMethod<T: Config> {
	controller: T::DidIdentifier,
}

#[derive(Decode, Encode, PartialEq, TypeInfo, MaxEncodedLen)]
pub struct KeyAgreement<T: Config> {
	controller: T::DidIdentifier,
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
pub struct Service<T: Config> {
	type_id: BoundedVec<u8, T::MaxString>, // E.g: IPFS
	service_endpoint: BoundedVec<u8, T::MaxString>, // E.g: IPFS endopoint
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
pub struct Document<T: Config> {
	controller: AccountIdOf<T>,
	authentication: Authentication<T>,
	assertion_method: Option<AssertionMethod<T>>,
	key_agreement: Option<KeyAgreement<T>>,
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
