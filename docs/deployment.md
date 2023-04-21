Ansible deployment guide
====================

Ansible is a popular tool to detect if a host matches the current expected state and also apply any changes or updates if required. Lots of pre-packaged modules exist in [ansible galaxy](https://galaxy.ansible.com/). 

The paritytech.chain_operations module is maintained on [github](https://github.com/paritytech/ansible-galaxy) and available via [ansible-galaxy]()


### Installation

Create a `requirements.yml` file and use `ansible-galaxy` to download the latest version. See the [chain_operations README.md](https://github.com/paritytech/ansible-galaxy/blob/main/README.md) for setup instructions.


### Deploy Watr Node

Goal: Deploy two relaychain validators and one parachain collator.

It expects a relaychain and parachain chain specifications. A [guide](https://docs.substrate.io/reference/how-to-guides/parachains/connect-to-a-relay-chain/) on how to create these chain specifications is available. This network will use default `Alice` and `Bob` keys.

#### Development Network Inventory:

Below are examples of each node role (bootnode / collator / rpc node / fullnode ):

```yaml
all:
  vars:
    node_binary_version: v0.9.37
    node_app_name: watr
    node_chainspec: polkadot
    node_parachain_chainspec: https://raw.githubusercontent.com/Watr-Protocol/watr/main/chain-specs/devnet-raw.json # chain spec file if required
    node_user: polkadot
    node_binary: "https://github.com/Watr-Protocol/watr/releases/download/v1.2.0/watr-node" # release binary to run on all nodes

  children:
    bootnodes: # example bootnode with injected p2p private key
      hosts:
        bootnode-1:
          node_parachain_role: boot
          node_p2p_private_key: "0x0" # inject p2p private key on bootnodes
          ansible_host: bootnode-1.mycompany.com

    collators: # example collator with injected aura private key
      hosts:
        collator-1:
          node_parachain_role: collator
          node_custom_options: ["--execution wasm"]
          key_inject_parachain_aura_private_key: "0x0" # inject this private aura key
          ansible_host: collator-1.mycompany.com

    rpcs: # example collator node
      hosts:
        rpc-1:
          node_parachain_role: rpc
          node_custom_options: ["--execution wasm"]
          ansible_host: rpc-1.mycompany.com

    fullnodes: # example parachain full node
      hosts:
        fullnode-1:
          node_parachain_role: full
          node_custom_options: ["--execution wasm"]
          ansible_host: fullnode-1.mycompany.com
```


#### Deployment Playbook:

```yaml
---
- name: deploy all nodes
  hosts: all
  become: yes
  roles:
    - parity.chain.node
    ---
- name: inject keys on parachain collators
  hosts: collators
  become: yes
  roles:
    - parity.chain.inject_keys
```


#### Cloudwatch Filter Example

```
fields @timestamp, @message
| sort @timestamp desc
| filter @message like /(?i)(Exception|error|fail|warning|warn)/ and @message not like "[Relaychain] Re-finalized block" and @message not like "RequestCollation" and @message not like "CannotUpgrade" and @message not like /WS send error: Networking or low-level protocol error/
```
