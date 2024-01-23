# de-cart

De-cart is a fully on-chain decentralised e-commerce application hosted on 'Internet Computer Protocol's (ICP) smart contract platform. This project was created to support my MSc research paper.

The purpose of this research paper was to explore how an e-commmerce smart contract could encrypt sensitive data within a smart contract's state, thus opening up new use-cases that are currently not possible. Examples of such use cases:

- Merchant API keys
- Shopping Carts
- Delivery address / Fulfilment details

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

## Installation of the following is required to run this project

Rust - https://www.rust-lang.org/tools/install

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

DFX - https://internetcomputer.org/docs/current/developer-docs/setup/install/

`sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"`

## Running the project locally

If you want to run the project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# The vetkd system api is not yet available on the main net so this system_api needs to be deployed as a cainster locally
dfx canister create system_api --specified-id s55qq-oqaaa-aaaaa-aaakq-cai

# Deploys your canisters to the replica and generates your candid interface
dfx deploy --network=local
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
