<p align="center">
	<a href="https://www.watr.org/">
		<img src="https://user-images.githubusercontent.com/23270067/213279777-545afe00-7353-47d8-a6f1-657490e39665.svg" width="350"/>
	</a>
</p>

<div align="center">
	
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Ftwitter.com%WatrProtocol)](https://twitter.com/WatrProtocol)
[![Medium](https://img.shields.io/badge/Medium-gray?logo=medium)](https://medium.com/watr-protocol)

</div>

# Watr
Watr Protocol is Watr’s decentralized and public blockchain platform that is open to everyone who wants to build, create and collaborate on it. Watr gives developers, entrepreneurs and investors direct access to commodities as a platform.
It is a Polkadot Parachain, leveraging the shared security of the Polkadot ecosystem as well as high transaction throughput, connectivity with the other parachains and regular upgrades.

## Building & Running Locally

- Before starting, please follow the Substrate quick start guide to setup the environment. https://docs.substrate.io/quick-start/
- Also, install zombienet: https://github.com/paritytech/zombienet

1. Clone and build the Watr node
	```shell
	git clone https://github.com/Watr-Protocol/watr.git
	cd watr

	# Build. Be patient, it can take a long time :)
	cargo build --release
	```

2. Build Polkadot 
	```shell
	git clone https://github.com/paritytech/polkadot.git
	cd polkadot

	# Build with fast-runtime enabled
	cargo build --release --features fast-runtime
	``` 

3. Copy the `polkadot` binary stored at `target/release/polkadot` into the `watr/bin` directory.

	```shell
	# In polkadot root. Assuming watr is one directory up.
	cp target/release/polkadot ../watr/bin
	```

4. Start the local testnet

	To start Mainnet:
	```shell
	zombienet -p native spawn zombienet-config/mainnet.toml
	```

	To start Devnet:
	```shell
	zombienet -p native spawn zombienet-config/devnet.toml
	```

## Run Tests
```shell
cargo test
```

## Guides
- [Benchmarks](docs/benchmarks.md)
- Release Guidelines: TODO
- [Integrations Tests](docs/integration-tests.md)
- [Governance](docs/governance/watr-governance-guide.md)
- Connecting Metamask: TODO
- [Collator Selection Reward Pot](docs/collator-selection-pot.md)

## Runtime Details
**Substrate**
- `pallet-scheduler` allows extrinsic calls to be scheduled for a later time
- `pallet-balances` maintains the native Watr currency
- `pallet-sudo` provides a single Root-privileged account. Will be removed
- `pallet-multisig` allows for several accounts to manage a single multisig account
- `pallet-identity` is a simple, federated, identity system that allows users to add a nickname, social medias, and more
- `pallet-collective` creates the Council
- `pallet-motion` provides root-level origin for the Council
- `pallet-membership` makes managing Council members easier
- `pallet-treasury` provides a pot for holding Council governed funds
- `pallet-assets` creates and manages new tokens
- `pallet-utility` provides dispatch management (such as batched calls)

**EVM / Frontier**
- `pallet-ethereum` provides Ethereum compatibilty and RPCs
- `pallet-evm` Adds an Ethereum Virtual Machine. Provides support for EVM contracts
- `pallet-base-fee` follows EIP-1559's fee mechanism
- `pallet-evm-precompile-assets-erc20` (aka XC-20s) allows EVM smart contracts to access `pallet-assets` using an ERC-20 interface
  
**XCM**
- `cumulus-pallet-xcmp-queue`
- `pallet-xcm`
- `cumulus-pallet-xcm`
- `cumulus-pallet-dmp-queue`
- XCM is configured to allow certain asset transfers to and from Statemint (e.g., USDt)

## Devnet
A current Devnet is running on the Rococo relay chain.

Endpoint: [https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.dev.watr.org%3A443#/explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.dev.watr.org%3A443#/explorer)

## Cumulus
This project was originally forked from the Substrate Parachain Template.

The stand-alone version of this template is hosted on the
[Substrate Devhub Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/)
for each release of Polkadot. It is generated directly to the upstream
[Parachain Template in Cumulus](https://github.com/paritytech/cumulus/tree/master/parachain-template)
at each release branch using the
[Substrate Template Generator](https://github.com/paritytech/substrate-template-generator/).

👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and
parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).


🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).
