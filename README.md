# de-cart

De-cart is a decentralised e-commerce application that is hosted fully onchain using the Internet Computer Protocol (ICP). This project was created to support my MSc research paper titled 'Methods for encrpyting sensitive data within e-commerce smart contracts'.

The purpose of this research paper was to explore how a an e-commmerce smart contract could encrypt sensitive data within a smart contract's state, thus opening up new use-cases that are currently not possible. Examples of such use cases:
- Merchant API keys
- Shopping Carts
- Delivery address / Fulfilment details  

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with de-cart, see the following documentation available online:


## Running the project locally

If you want to run the project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
