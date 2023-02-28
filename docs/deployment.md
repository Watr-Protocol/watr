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

```yaml
all:
  vars:
    node_binary_version: v0.9.37
    node_app_name: watr
    node_chainspec: polkadot
    node_parachain_chainspec: https://raw.githubusercontent.com/Watr-Protocol/watr/main/chain-specs/devnet-raw.json
    node_user: polkadot
    node_binary: "https://github.com/Watr-Protocol/watr/releases/download/v1.2.0/watr-node"

  children:
    collator:
      hosts:
        collator-1:
          node_parachain_role: collator
          node_custom_options: ["--execution wasm"]
          ansible_host: collator-1.mycompany.com
```


#### Deployment Playbook:

```yaml
---
- name: deploy all nodes
  hosts: all
  become: yes
  roles:
    - parity.chain.node
```


#### Cloudwatch Filter Example

```
fields @timestamp, @message
| sort @timestamp desc
| filter @message like /(?i)(Exception|error|fail|warning|warn)/ and @message not like "[Relaychain] Re-finalized block" and @message not like "RequestCollation" and @message not like "CannotUpgrade" and @message not like /WS send error: Networking or low-level protocol error/
```
