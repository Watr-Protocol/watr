# Watr Identity Pallet

# Proposal

## Actors

### Subject

- Identity related to a DID

### User

- **Buyer**: The *subject* that wants to buy commodities
- **Seller**: The *subject* that wants to sell commodities

### Issuer

- An accredited *subjec*t appointed by Governance that can issue `VerifiableCredential`

### Governance

- For now composed by the Watr Council

### Market Place

- Platform where *Buyers* and *Sellers* discover each other and can see their mutual credentials

## Components

### DID

Unique identifier

- **DID method** → `watr`
- **DID method-specific identifier** → Substrate account address

E.g: `did:watr:Ff5P54C4ockeSZrUJ2CLsRNR6CesAwjYZEN2yjh5SU2xu7R`

### DID resolver

Mechanism by which you can relate a DID with its document

### Document

It holds information of the DID in a 1 to 1 relationship. It includes:

- DID controller
    - DID that can modify the Document
- Authentication methods
    - Method to prove the ownership of a private key
- Attestation methods
    - Method to prove the ownership of a private key for signing attestations
- Agreement methods
    - Key pair for sending encrypted messages

```rust
struct Authentication<T> {
	controller: DidIdentifierOf<T>,
}

struct AssertionMethod<T> {
	controller: DidIdentifierOf<T>,
}

struct KeyAgreement<T> {
	controller: DidIdentifierOf<T>,
}

struct Service<T> {
	type_id: BoundedVec<u8, T::MaxString>, // E.g: IPFS
	service_endpoint: BoundedVec<u8, T::MaxString>, // E.g: IPFS endopoint
}

struct Document<T> {
	controller: DidIdentifierOf<T>,
	authentication: Authentication, // Default to Origin signer upon creation
	assertion_method: Option<AssertionMethod<T>>,
	key_agreement: Option<KeyAgreement<T>>,
	services: Options<Vec<Service<T>>>,
}
```

<aside>
📝 We simplify `Document` schema for the sake of reducing unnecessary storage on-chain

</aside>

<aside>
📝 For all cryptographic methods we use ****`SR25519`** as it is the one used by default by Substrate by the accounts

</aside>

<aside>
📝 We create `Authentication`, `AssertionMethod`, and `KeyAgreement` structs in case in the future they want to be extended with the rest of the expected W3C standard fields. For example specifying different cryptographic `type` (`Ed25519VerificationKey2020`...) or to point to `VerificationMethod` list

</aside>

### Verifiable Credentials

Are stored in a distributed storage system (E.g: IPFS) where the *subject* information remains private by encrypting it with the *subject* Public Key

Having the credentials encrypted split in many types helps with selective disclosure. A user could prove, for example, that he or she is not a USA citizen without exposing other information that can be included inside another credential (”KYCpassed”, for example).

If a verifier needs more than the Credential, the user could share the encrypted content inside using their Public Key plus the proof of validity.

```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
  ],
  "type": [
    "VerifiableCredential",
		"KYCpassed",
		"NotUSAcitizen",
		"NotForbiddenCountry"
  ],
  "issuer": "did:watr:Ff5P54C4ockeSZrUJ2CLsRNR6CesAwjYZEN2yjh5SU2xu7R",
  "issuanceDate": "2018-02-24T05:28:04Z",
	"validUntilBlock": "1500000"
  "credentialSubject": {
    "id": "did:watr:EPStAMtjApGg8Ap6xKe9gyuinjmetz1MNhzu1cPmLQkWKUA",
		"KYCpassed": "A3zpczxPy2fDzqv9Pgr4XBzX2rys1FDuLNkYRVmhXuyype8fB44qNX8m
NnXf99i7x1eSpLdYKNhEKknEJmdGfQ4w", //Encrypted information
		"NotUSAcitizen": "B4zpczxPy2fDzqv9Pgr4XBzX2rys1FDuLNkYRVmhXuyype8fB44qNX8m
NnXf99i7x1eSpLdYKNhEKknEJmdGfQ5c", //Encrypted information
		"NotForbiddenCountry": "C5zpczxPy2fDzqv9Pgr4XBzX2rys1FDuLNkYRVmhXuyype8fB44qNX8m
NnXf99i7x1eSpLdYKNhEKknEJmdGfQ6l", //Encrypted information
  },
  "proof": {
    "type": "Sr25519Signature2020",
    "created": "2022-02-25T14:58:43Z",
    "proofPurpose": "assertionMethod",
    "proofValue": "zyrpmzxPy2fDzqv9Pgr4XBzX2rys1FDuLNkYRVmhXuyype8fB44qNX8m
NnXf99i7x1eSpLdYKNhEKknEJmdGfQ4w"
  }
}
```

The Issuer will store anchor events when issuing credentials via signing and sending an Extrinsic

```rust
CredentialsIssued { 
	issuer: DidIdentifierOf<T>, //Ff5P54C4ockeSZrUJ2CLsRNR6CesAwjYZEN2yjh5SU2xu7R
	subject: DidIdentifierOf<T>, //EPStAMtjApGg8Ap6xKe9gyuinjmetz1MNhzu1cPmLQkWKUA
	credentials: Vec<Vec<u8>>, //[ KYCPassed, NotUSAcitizen, NotForbiddenCountry ]
	verifiable_credential_hash: Hash,
},
```

This events will be stored by indexing services that will be used by the Market Place to read and display *subjects* credentials

In addition, some of the information will be stored on-chain:

```rust
CredentialInfo {
	verifiable_credential_hash: Hash,
}

// StorageDoubleMap
Credentials[Subject][Credential][Issuer] -> CredentialInfo
```

## Configuration

```rust
#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type for a DID subject identifier.
		type DidIdentifier: Parameter + DidVerifiableIdentifier + MaxEncodedLen;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The amount held on deposit for a DID creation
		#[pallet::constant]
		type DidDeposit: Get<BalanceOf<Self>>;

		/// Maxmimum length for Credential types names
		#[pallet::constant]
		type MaxString: Get<u32>;

		/// Maxmimum number of Serivice per DID
		#[pallet::constant]
		type MaxServices: Get<u32>;

		/// The origin which may forcibly perform root actions
		type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}
```

## Storage

### Did

```rust
/// DID Resolver
#[pallet::storage]
pub type Did<T: Config> = StorageMap<
	_,
	Blake2_128Concat,
	DidIdentifierOf<T>, // Subject
	Document<T>, // Document
	ValueQuery,
>;
```

### Credentials

```rust
struct CredentialInfo<T> {
	verifiable_credential_hash: Hash,
}

#[pallet::storage]
pub type Credentials<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, DidIdentifierOf<T>>, // Subject
			NMapKey<Blake2_128Concat, BoundedVec<u8, T::MaxString>, // Credential
			NMapKey<Blake2_128Concat, DidIdentifierOf<T>>, // Issuer
		),
		CredentialInfo<T>,
	>;
```

### ValidCredentialsTypes

```rust
#[pallet::storage]
// list of valid Credentials types
pub(super) type ValidCredentialsTypes<T: Config> = StorageValue<
	_,
	BoundedVec<BoundedVec<u8, T::MaxString>>, T::MaxCredentialTypes>,
	ValueQuery,
>;
```

### Issuers

```rust
enum IssuerStatus {
	Active,
	Revoked
}
struct IssuerInfo {
	status: IssuerStatus
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
```

## Extrinsics

```rust
fn create_did(authentication, key_agreement, services) {
 // Reserve deposit
}

fn delete_did() {
 // Gets deposit back
 // Do not allow if it is a still an Active Issuer
}

fn update_did_document(controller, authentication, key_agreement, services) {
	// Origin ONLY controller
}

fn force_update_did_document(controller, authentication, key_agreement, services) {
	// Origin ONLY GovernanceOrigin
}

fn add_issuer(issuer) {
	// Origin ONLY GovernanceOrigin
	// Add issuer to database with status Active
}

fn revoke_issuer(issuer) {
 // Origin ONLY GovernanceOrigin
 // Change status to Revoked
}

fn remove_issuer(issuer) {
 // Origin ONLY GovernanceOrigin
 // Remove from database
 // Can be called when a Issuer is deregistered()
}

fn issue_credentials(subject, credentials, storage_hash) {
 // Origin ONLY valid Issuer
}

fn revoke_credentials(subject, credentials, storage_hash) {
 // Origin ONLY valid Issuer
 // Credential will be deleted
 // It has to also emit an Event to let the indexes services know the Credential
 // was revoked
}

fn add_credential_type()
 // Origin ONLY GovernanceOrigin

fn remove_credential_type()
 // Origin ONLY GovernanceOrigin

```

## Usage Flow

1. Users and Issuers create DIDs using `create_did`
    1. controller: the DID that is allowed to modify the DID being created
    2. authentication: a public key (by default, an EVM / H160 address) used to prove ownership of the DID
    3. assertion: optional field used for Issuers to validate assertions an issuer makes on another DID
    4. services: endpoints (e.g. ipfs) that the issuer uses to store verifiable credentials
2. GovernanceOrigin adds credential types using `add_credentials_type`
    1. These credentials should be statements such as “KYCPassed”, “NotFromUS”, “A-Rating”, etc.
3. GovernanceOrigin adds an issuer by using `add_issuer(issuer_did)`
4. Issuer calls `issue_credentials` to issue credentials to a DID
    1. credentials: a vector of credentials issued to the subject DID
    2. `verifiable_credential_hash` is a hash of the *entire* verifiable credential. The actual verifiable credential will be stored off-chain
5. A verifier should validate a DID’s credentials on-chain and off-chain through the verifiable credential
    1. A verifier MUST trust an issuer of credentials
    2. A verifier MUST validate the issuer status on chain
        1. If the issuer is revoked, or deleted, the verifier may not trust a credential / attestation given by the disabled issuer

## User Flow

1. *User* (both *Buyer* and *Seller*) **`create_did()` and reserve deposit
2. Go through the Verifiable Credential provider
3. When done with verification (KYC for example) user encrypt the provided data and at the same time proves he or she is the owner of the DID (should match with a DID account with some reserved deposit)
4. Verifiable Credential Provider
    1. Must create its own DID first AND governance must `add_issuer(issuer_did)`
    2. generates the `VerifiableCredential` and uploads it to Distributed Storage service (E.g: IPFS)
    3. call `issue_credentials()` emitting an Event that is listened by Indexing Services
5. User logs in in the market place
6. *Seller* finds a *Buyer* or vice versa.
    1. Their Credentials information is shown thanks to querying indexing services
    2. The credentials themselves can be found thanks to the `storage_hash` and the `Document::service` field where the storage service endpoint is defined
7. If any party needs to know more details from the other, the other party can always disclose the Credential information sharing the information plus their Public Key

## Assumptions

To simplify the solution for a first and fast development iteration:

- There are not Delegations (1 to 1 relationship without a hierarchy system between `Document` and `VerifiableCredential` controllers)
- For all cryptographic methods we use ****`SR25519`** as it is the one used by default by Substrate by the accounts
- No need to make it super generic as it is a custom solution and not meant to be used by 3rd parties (it can be done in future iterations)

## Edge cases

- User lose their private keys
    - Governance can modify the `Document` controller
        - Need for a  dispatchable (`did.force_update_did_document`)

## Additions for the Future

- `valid_until` for a credential
    - automatic credential removal