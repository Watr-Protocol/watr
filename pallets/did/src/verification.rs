use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{ensure, RuntimeDebug};
use scale_info::TypeInfo;
use sp_core::{ecdsa, ed25519, sr25519, H160, H256};
use sp_runtime::traits::Verify;
use sp_std::convert::TryInto;

use crate::{errors::SignatureError, Payload};

/// Types of verification keys a DID can control.
#[derive(
	Clone, Decode, RuntimeDebug, Encode, Eq, Ord, PartialEq, PartialOrd, TypeInfo, MaxEncodedLen,
)]
pub enum DidVerificationKey {
	/// An Ed25519 public key.
	Ed25519(ed25519::Public),
	/// A Sr25519 public key.
	Sr25519(sr25519::Public),
	/// An ECDSA public key.
	Ecdsa(ecdsa::Public),
}

impl DidVerificationKey {
	/// Verify a DID signature using one of the DID keys.
	pub fn verify_signature(
		&self,
		payload: &Payload,
		signature: &DidSignature,
	) -> Result<(), SignatureError> {
		match (self, signature) {
			(DidVerificationKey::Ed25519(public_key), DidSignature::Ed25519(sig)) => {
				ensure!(sig.verify(payload, public_key), SignatureError::InvalidSignature);
				Ok(())
			},
			// Follows same process as above, but using a Sr25519 instead
			(DidVerificationKey::Sr25519(public_key), DidSignature::Sr25519(sig)) => {
				ensure!(sig.verify(payload, public_key), SignatureError::InvalidSignature);
				Ok(())
			},
			(DidVerificationKey::Ecdsa(public_key), DidSignature::Ecdsa(sig)) => {
				ensure!(sig.verify(payload, public_key), SignatureError::InvalidSignature);
				Ok(())
			},
			_ => Err(SignatureError::InvalidSignatureFormat),
		}
	}
}

impl From<ed25519::Public> for DidVerificationKey {
	fn from(key: ed25519::Public) -> Self {
		DidVerificationKey::Ed25519(key)
	}
}

impl From<sr25519::Public> for DidVerificationKey {
	fn from(key: sr25519::Public) -> Self {
		DidVerificationKey::Sr25519(key)
	}
}

impl From<ecdsa::Public> for DidVerificationKey {
	fn from(key: ecdsa::Public) -> Self {
		DidVerificationKey::Ecdsa(key)
	}
}

/// Types of signatures supported by this pallet.
#[derive(Clone, Decode, RuntimeDebug, Encode, Eq, PartialEq, TypeInfo)]
pub enum DidSignature {
	/// A Ed25519 signature.
	Ed25519(ed25519::Signature),
	/// A Sr25519 signature.
	Sr25519(sr25519::Signature),
	/// An Ecdsa signature.
	Ecdsa(ecdsa::Signature),
}

impl From<ed25519::Signature> for DidSignature {
	fn from(sig: ed25519::Signature) -> Self {
		DidSignature::Ed25519(sig)
	}
}

impl From<sr25519::Signature> for DidSignature {
	fn from(sig: sr25519::Signature) -> Self {
		DidSignature::Sr25519(sig)
	}
}

impl From<ecdsa::Signature> for DidSignature {
	fn from(sig: ecdsa::Signature) -> Self {
		DidSignature::Ecdsa(sig)
	}
}

pub trait DidVerifiableIdentifier {
	/// Allows a verifiable identifier to verify a signature it produces and
	/// return the public key
	/// associated with the identifier.
	fn verify_and_recover_signature(
		&self,
		payload: &Payload,
		signature: &DidSignature,
	) -> Result<DidVerificationKey, SignatureError>;
}

impl DidVerifiableIdentifier for H160 {
	fn verify_and_recover_signature(
		&self,
		payload: &Payload,
		signature: &DidSignature,
	) -> Result<DidVerificationKey, SignatureError> {
		// So far, either the raw Ed25519/Sr25519 public key or the Blake2-256 hashed
		// ECDSA public key.
		let raw_public_key: &[u8; 20] = self.as_fixed_bytes();

		Err(SignatureError::InvalidSignature)

		// TODO: implement for 20bytes Ethereum address, aka H160
		// match *signature {
		// 	DidSignature::Ecdsa(ref signature) => {
		// 		let ecdsa_signature: [u8; 65] = signature
		// 			.encode()
		// 			.try_into()
		// 			.map_err(|_| SignatureError::InvalidSignature)?;
		// 		// ECDSA uses blake2-256 hashing algorithm for signatures, so we hash the given
		// 		// message to recover the public key.
		// 		let hashed_message = sp_io::hashing::blake2_256(payload);
		// 		let recovered_pk: [u8; 33] =
		// 			sp_io::crypto::secp256k1_ecdsa_recover_compressed(&ecdsa_signature, &hashed_message)
		// 				.map_err(|_| SignatureError::InvalidSignature)?;
		// 		let hashed_recovered_pk = sp_io::hashing::blake2_256(&recovered_pk);
		// 		// The hashed recovered public key must be equal to the AccountId32 value, which
		// 		// is the hashed key.
		// 		ensure!(&hashed_recovered_pk == raw_public_key, SignatureError::InvalidSignature);
		// 		// Safe to reconstruct the public key using the recovered value from
		// 		// secp256k1_ecdsa_recover_compressed
		// 		Ok(DidVerificationKey::from(ecdsa::Public(recovered_pk)))
		// 	}
		// }
	}
}

impl DidVerifiableIdentifier for H256 {
	fn verify_and_recover_signature(
		&self,
		payload: &Payload,
		signature: &DidSignature,
	) -> Result<DidVerificationKey, SignatureError> {
		// So far, either the raw Ed25519/Sr25519 public key or the Blake2-256 hashed
		// ECDSA public key.
		let raw_public_key: &[u8; 32] = self.as_fixed_bytes();
		match *signature {
			DidSignature::Ed25519(_) => {
				// from_raw simply converts a byte array into a public key with no particular
				// validations
				let ed25519_did_key =
					DidVerificationKey::Ed25519(ed25519::Public::from_raw(*raw_public_key));
				ed25519_did_key.verify_signature(payload, signature).map(|_| ed25519_did_key)
			},
			DidSignature::Sr25519(_) => {
				let sr25519_did_key =
					DidVerificationKey::Sr25519(sr25519::Public::from_raw(*raw_public_key));
				sr25519_did_key.verify_signature(payload, signature).map(|_| sr25519_did_key)
			},
			DidSignature::Ecdsa(ref signature) => {
				let ecdsa_signature: [u8; 65] =
					signature.encode().try_into().map_err(|_| SignatureError::InvalidSignature)?;
				// ECDSA uses blake2-256 hashing algorithm for signatures, so we hash the given
				// message to recover the public key.
				let hashed_message = sp_io::hashing::blake2_256(payload);
				let recovered_pk: [u8; 33] = sp_io::crypto::secp256k1_ecdsa_recover_compressed(
					&ecdsa_signature,
					&hashed_message,
				)
				.map_err(|_| SignatureError::InvalidSignature)?;
				let hashed_recovered_pk = sp_io::hashing::blake2_256(&recovered_pk);
				// The hashed recovered public key must be equal to the AccountId32 value, which
				// is the hashed key.
				ensure!(&hashed_recovered_pk == raw_public_key, SignatureError::InvalidSignature);
				// Safe to reconstruct the public key using the recovered value from
				// secp256k1_ecdsa_recover_compressed
				Ok(DidVerificationKey::from(ecdsa::Public(recovered_pk)))
			},
		}
	}
}
