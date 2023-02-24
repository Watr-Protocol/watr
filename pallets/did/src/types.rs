use super::*;
use codec::{Decode, Encode, MaxEncodedLen, WrapperTypeEncode};
use frame_support::pallet_prelude::{CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use scale_info::TypeInfo;
use sp_runtime::{ArithmeticError, RuntimeDebug};

/// Type used to count the number of references a service has.
pub type RefCount = u32;

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
	VerifiableCredentialFileStorage,
}

#[derive(
	CloneNoBound,
	PartialEqNoBound,
	Default,
	Decode,
	Encode,
	RuntimeDebugNoBound,
	TypeInfo,
	MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct Service<T: Config> {
	type_id: ServiceType,
	pub service_endpoint: BoundedVec<u8, T::MaxString>,
	// count of DIDs referencing this service
	consumers: RefCount,
}

#[derive(
	CloneNoBound, PartialEqNoBound, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,
)]
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
	Revoked,
}

#[derive(Clone, Decode, Default, Encode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct IssuerInfo {
	pub status: IssuerStatus,
}

impl<T: Config> Service<T> {
	// Increment service consumers count. Returns error upon overflow.
	pub fn inc_consumers(&mut self) -> Result<(), ArithmeticError> {
		self.consumers.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		Ok(())
	}

	// Decrement service consumers count. Can not underflow
	pub fn dec_consumers(&mut self) {
		if self.consumers > 0 {
			self.consumers - 1;
		}
	}

	pub fn consumers(&mut self) -> RefCount {
		self.consumers
	}
}