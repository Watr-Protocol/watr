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

use super::*;
use frame_support::pallet_prelude::{CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
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
pub enum ServiceType {
	#[default]
	VerifiableCredentialFileStorage,
}

#[derive(
	CloneNoBound, PartialEqNoBound, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct Service<T: Config> {
	pub info: ServiceInfo<T>,
	/// count of DIDs referencing this service
	consumers: RefCount,
}

#[derive(
	CloneNoBound, PartialEqNoBound, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct ServiceInfo<T: Config> {
	pub type_id: ServiceType,
	pub service_endpoint: BoundedVec<u8, T::MaxString>,
}

#[derive(
	CloneNoBound, PartialEqNoBound, Decode, Encode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct Document<T: Config> {
	pub controller: DidIdentifierOf<T>,
	pub authentication: AuthenticationMethod<T>,
	pub assertion_method: Option<AssertionMethod<T>>,
	pub services: BoundedVec<KeyIdOf<T>, T::MaxServices>,
}

#[derive(Clone, Decode, Default, Encode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub enum IssuerStatus {
	#[default]
	Active,
	Revoked,
	Deleted,
}

#[derive(Clone, Decode, Default, Encode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct IssuerInfo {
	pub status: IssuerStatus,
}

impl<T: Config> Service<T> {
	pub fn new(info: ServiceInfo<T>) -> Self {
		Service {
			info,
			// start at 1 because a did is creating this service
			consumers: 1,
		}
	}
	// Increment service consumers count. Returns error upon overflow.
	pub fn inc_consumers(&mut self) -> Result<(), ArithmeticError> {
		self.consumers = self.consumers.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		Ok(())
	}

	// Decrement service consumers count. Can not underflow
	pub fn dec_consumers(&mut self) {
		if self.consumers > 0 {
			self.consumers -= 1;
		}
	}

	pub fn consumers(&self) -> RefCount {
		self.consumers
	}
}

/// Witness Services
#[derive(
	Copy, Clone, Default, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo,
)]
pub struct ServicesWitness {
	/// The number of Services inserts
	#[codec(compact)]
	pub inserts: u32,
	/// The number of Services removals
	#[codec(compact)]
	pub removals: u32,
}

#[derive(
	Clone, Decode, Default, Encode, PartialEq, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct CredentialInfo<T: Config> {
	pub verifiable_credential_hash: HashOf<T>,
}
