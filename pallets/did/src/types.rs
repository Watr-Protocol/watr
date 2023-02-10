use super::*;

struct Authentication<T: Config> {
	controller: T::DidIdentifier,
}

struct AssertionMethod<T: Config> {
	controller: T::DidIdentifier,
}

struct KeyAgreement<T: Config> {
	controller: T::DidIdentifier,
}

struct Service<T: Config> {
	type_id: BoundedVec<u8, T::MaxString>, // E.g: IPFS
	service_endpoint: BoundedVec<u8, T::MaxString>, // E.g: IPFS endopoint
}

struct Document<T: Config> {
	controller: AccountIdOf<T>,
	authentication: Authentication<T>,
	assertion_method: Option<AssertionMethod<T>>,
	key_agreement: Option<KeyAgreement<T>>,
	services: Option<Vec<Service<T>>>,
}
