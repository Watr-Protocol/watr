# Integration Tests

1. Install `parachains-integration-tests` [npm package](https://www.npmjs.com/package/@parity/parachains-integration-tests) - [Github](https://github.com/paritytech/parachains-integration-tests) repo
	```
	yarn global add @parity/parachains-integration-tests
	```
2. Install locally particular npm package dependancies required by the integration tests
	```
	yarn
	```
3. `polkadot`, `polkadot-parachain` and `watr-node` compiled binaries should be copied to `./bin` folder
	- The `polkadot-parachain` should be compiled from one of the parachains release branches from Cumulus. Ans example is this [branch](https://github.com/paritytech/cumulus/tree/release-parachains-v9360)
	- The `polkadot` binary should be compiled with `fast-runtime` feature from a release branch with `sudo` pallet on it. An example is this [branch](https://github.com/paritytech/polkadot/tree/it/release-v0.9.36-fast-sudo)
		```
		cargo build --release --features fast-runtime
		```
4. From the repository root folder, run:
	- For Devnet:
		```
		parachains-integration-tests -m zombienet-test -t ./integration-tests/runtimes/devnet/ -c ./integration-tests/runtimes/devnet/config.toml
		```
	- For Mainnet:
		```
		parachains-integration-tests -m zombienet-test -t ./integration-tests/runtimes/mainnet/ -c ./integration-tests/runtimes/mainnet/config.toml
		```
4. Wait until all tests pass
