# Send balances between Substrate and EVM layers

Substrate (`SS58`) and Ethereum (`H160`) use different addresses. Because of this, address transformations are required to use an EVM within Substrate.

[This tool](https://hoonsubin.github.io/evm-substrate-address-converter/) can be used for an easy method to translate an Ethereum address to Substrate and vice versa.
Note that prefix has to be set to be 19 (Watr's)

Let's imagine a user with two wallets with their respective private keys:
- Substrate A (private key)
- EVM B (private key)

1. How to fund your EVM B wallet:
- EVM B transformation to Substrate B
- Send tokens from Substrate A to Substrate B -> EVM B gets the funds

2. How to get your tokens back to Substrate A:
- Substrate A transformation to EVM A
- Send tokens from EVM B to EVM A
- Tokens are not received in Substrate A, they will be accounted to Substrate C (EVM A transformation to Substrate)
- However, the tokens will be available to be withdrawn by Substrate A from Substrate C since they are in its corresponding EVM A.
- Sign and send by Substrate A -> `evm.withdraw(address: EVM A, value)`
