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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod types;
mod verification;
mod errors;

use sp_std::prelude::*;
use sp_core::H160;
use frame_support::{
	BoundedVec,
	dispatch::{DispatchResult, Dispatchable, GetDispatchInfo, PostDispatchInfo},
	ensure,
	storage::types::StorageMap,
	traits::{Get, OnUnbalanced, WithdrawReasons},
	Parameter,
};
use crate::verification::{DidSignature};
use crate::types::{IssuerInfo};

pub use pallet::*;
use verification::{DidVerifiableIdentifier};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	/// Reference to a payload of data of variable size.
	pub type Payload = [u8];

	// /// Type for a DID key identifier.
	// pub type KeyIdOf<T> = <T as frame_system::Config>::Hash;

	/// Type for a DID subject identifier.
	pub type DidIdentifierOf<T> = <T as Config>::DidIdentifier;

	/// Type for valid Credentials.
	pub type CredentialOf<T> = BoundedVec<u8, <T as Config>::MaxString>;

	/// Type for Watr account identifier.
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	/// Type for a block number.
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	// /// Type for a runtime extrinsic callable under DID-based authorisation.
	// pub type DidCallableOf<T> = <T as Config>::RuntimeCall;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		/// Type for a DID subject identifier.
		type DidIdentifier: Parameter + DidVerifiableIdentifier + MaxEncodedLen;

		// Origin for priviledged actions
		type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The maximum length of a service ID.
		#[pallet::constant]
		type MaxString: Get<u32>;

		/// The maximum Credential types.
		#[pallet::constant]
		type MaxCredentialsTypes: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn issuers)]
	pub type Issuers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DidIdentifierOf<T>,
		IssuerInfo,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn credential_types)]
	// list of valid Credentials types
	pub(super) type CredentialsTypes<T: Config> = StorageValue<
		_,
		BoundedVec<CredentialOf<T>, T::MaxCredentialsTypes>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		DidCreated { controller: AccountIdOf<T>, authenticator: H160, signature: DidSignature }
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(1000000)]
		pub fn create_did(
			origin: OriginFor<T>,
			authenticator: H160, // Ethereum Address
			signature: DidSignature
		) -> DispatchResult {
			let controller = ensure_signed(origin)?;
			Self::deposit_event(Event::DidCreated{
				controller,
				authenticator,
				signature
			});
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn add_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			// Add issuer to database with status Active
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn revoke_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			// Change status to Revoked
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn delete_issuer(origin: OriginFor<T>, issuer: DidIdentifierOf<T>) -> DispatchResult {
			// Called by the user to de
			T::GovernanceOrigin::ensure_origin(origin)?;
			// Remove issuer from storage
			// Should be also called when a Issuer DID is deleted()
			// because of that prob create a do_delete_issuer() pallet helper method
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn add_credential_type(origin: OriginFor<T>, credential: CredentialOf<T>) -> DispatchResult {
 			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			Ok(())
		}

		#[pallet::weight(1000000)]
		pub fn remove_credential_type(origin: OriginFor<T>, credential: CredentialOf<T>) -> DispatchResult {
			// Origin ONLY GovernanceOrigin
			T::GovernanceOrigin::ensure_origin(origin)?;
			Ok(())
		}
	}
}
