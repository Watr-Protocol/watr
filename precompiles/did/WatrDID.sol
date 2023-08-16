// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.0;

interface WatrDID {
	struct OptionalAddress {
		bool hasValue;
		address value;
	}
	struct Service {
		uint8 typeId;
		string serviceEndpoint;
	}
	struct OptionalServices {
		bool hasValue;
		Service[] services;
	}
	function createDid(address controller, address authentication, OptionalAddress calldata assertion, Service[] calldata services) external;
	function updateDid(address did, OptionalAddress calldata controller, OptionalAddress calldata authentication, OptionalAddress calldata assertion, OptionalServices calldata services) external;
	function removeDid(address did) external;
	function addDidServices(address did, Service[] calldata services) external;
	function removeDidServices(address did, bytes[] calldata serviceKeys) external;
	function issueCredentials(address issuerDid, address subjectDid, string[] calldata credentials, bytes calldata verifiableCredentialHash) external;
	function revokeCredentials(address issuerDid, address subjectDid, string[] calldata credentials) external;
}
