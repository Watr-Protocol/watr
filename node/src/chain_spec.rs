use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::collections::BTreeMap;

use parachains_common::{AccountId, AuraId, Signature};
use watr_runtime as mainnet;
use watr_runtime::WATR;

use watr_devnet_runtime as devnet;
use watr_devnet_runtime::WATRD;

/// Specialized `MainnetChainSpec` for the normal parachain runtime.
pub type MainnetChainSpec = sc_service::GenericChainSpec<mainnet::GenesisConfig, Extensions>;

pub type DevnetChainSpec = sc_service::GenericChainSpec<devnet::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

pub const PARA_ID: u32 = 2058;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn mainnet_session_keys(keys: AuraId) -> mainnet::SessionKeys {
	mainnet::SessionKeys { aura: keys }
}

pub fn devnet_session_keys(keys: AuraId) -> devnet::SessionKeys {
	devnet::SessionKeys { aura: keys }
}

pub fn devnet_development_config() -> DevnetChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WATRD".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 19.into());

	DevnetChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			devnet_testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					//prefunded EVM account
					hex!["e1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d"].into(),
				],
				// initial councillors
				sp_runtime::bounded_vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
				],
				PARA_ID.into(),
				// Total supply
				Some(12000000 * WATRD),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

pub fn mainnet_development_config() -> MainnetChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WATR".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 19.into());

	MainnetChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			mainnet_testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					//prefunded EVM account
					hex!["e1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d"].into(),
				],
				// initial councillors
				sp_runtime::bounded_vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
				],
				PARA_ID.into(),
				// Total supply
				Some(12000000 * WATR),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

pub fn devnet_local_testnet_config() -> DevnetChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WATRD".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 19.into());

	DevnetChainSpec::from_genesis(
		// Name
		"Local Watr Devnet",
		// ID
		"local_watr_devnet",
		ChainType::Local,
		move || {
			devnet_testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					//prefunded EVM account
					hex!["e1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d"].into(),
				],
				// initial councillors
				sp_runtime::bounded_vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
				],
				PARA_ID.into(),
				// Total supply
				Some(12000000 * WATRD),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("watr-network"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

pub fn mainnet_local_testnet_config() -> MainnetChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WATR".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 19.into());

	MainnetChainSpec::from_genesis(
		// Name
		"Local Watr Mainnet",
		// ID
		"local_watr_mainnet",
		ChainType::Local,
		move || {
			mainnet_testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					//prefunded EVM account
					hex!["e1ad20aae239ccbb609aa537d515dc9d53c5936ea67d8acc9fe0618925279f7d"].into(),
				],
				// initial councillors
				sp_runtime::bounded_vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
				],
				PARA_ID.into(),
				// Total supply
				Some(12000000 * WATR),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("watr-network"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

fn devnet_testnet_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	councillors: sp_runtime::BoundedVec<AccountId, devnet::CouncilMaxMembers>,
	id: ParaId,
	total_issuance: Option<devnet::Balance>,
) -> devnet::GenesisConfig {
	use devnet::EVMConfig;

	let num_endowed_accounts = endowed_accounts.len();
	let balances = match total_issuance {
		Some(total_issuance) => {
			let balance_per_endowed = total_issuance
				.checked_div(num_endowed_accounts as devnet::Balance)
				.unwrap_or(0 as devnet::Balance);

			endowed_accounts.iter().cloned().map(|k| (k, balance_per_endowed)).collect()
		},
		None => vec![],
	};

	devnet::GenesisConfig {
		system: devnet::SystemConfig {
			code: devnet::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},

		balances: devnet::BalancesConfig { balances },

		parachain_info: devnet::ParachainInfoConfig { parachain_id: id },
		collator_selection: devnet::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: devnet::EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: devnet::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),               // account id
						acc,                       // validator id
						devnet_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		sudo: devnet::SudoConfig { key: Some(root_key.clone()) },
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		assets: devnet::AssetsConfig {
			assets: vec![(1984, root_key, true, 100_000)],
			metadata: vec![(1984, b"Tether USD".to_vec(), b"USDt".to_vec(), 6)],
			accounts: vec![],
		},
		parachain_system: Default::default(),
		polkadot_xcm: devnet::PolkadotXcmConfig { safe_xcm_version: Some(SAFE_XCM_VERSION) },

		council_membership: devnet::CouncilMembershipConfig {
			members: councillors,
			phantom: Default::default(),
		},
		treasury: Default::default(),

		// EVM compatibility
		evm: EVMConfig { accounts: { BTreeMap::new() } },
		ethereum: Default::default(),
		base_fee: Default::default(),
	}
}

fn mainnet_testnet_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	councillors: sp_runtime::BoundedVec<AccountId, mainnet::CouncilMaxMembers>,
	id: ParaId,
	total_issuance: Option<mainnet::Balance>,
) -> mainnet::GenesisConfig {
	use mainnet::EVMConfig;

	let num_endowed_accounts = endowed_accounts.len();
	let balances = match total_issuance {
		Some(total_issuance) => {
			let balance_per_endowed = total_issuance
				.checked_div(num_endowed_accounts as mainnet::Balance)
				.unwrap_or(0 as mainnet::Balance);

			endowed_accounts.iter().cloned().map(|k| (k, balance_per_endowed)).collect()
		},
		None => vec![],
	};

	mainnet::GenesisConfig {
		system: mainnet::SystemConfig {
			code: mainnet::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},

		balances: mainnet::BalancesConfig { balances },

		parachain_info: mainnet::ParachainInfoConfig { parachain_id: id },
		collator_selection: mainnet::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: mainnet::EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: mainnet::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                // account id
						acc,                        // validator id
						mainnet_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		sudo: mainnet::SudoConfig { key: Some(root_key.clone()) },
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		assets: mainnet::AssetsConfig {
			assets: vec![(1984, root_key, true, 100_000)],
			metadata: vec![(1984, b"Tether USD".to_vec(), b"USDt".to_vec(), 6)],
			accounts: vec![],
		},
		parachain_system: Default::default(),
		polkadot_xcm: mainnet::PolkadotXcmConfig { safe_xcm_version: Some(SAFE_XCM_VERSION) },

		council_membership: mainnet::CouncilMembershipConfig {
			members: councillors,
			phantom: Default::default(),
		},
		treasury: Default::default(),

		// EVM compatibility
		evm: EVMConfig { accounts: { BTreeMap::new() } },
		ethereum: Default::default(),
		base_fee: Default::default(),
	}
}
