# Aptos Wallet CLI

A simple command-line interface (CLI) tool for generating an Aptos wallet and checking its balance on the mainnet.

## Description

This is a basic Rust application that demonstrates how to:
-   Generate a new Aptos-compatible wallet (signing key and verifying key).
-   Derive a 32-byte account address from the public key.
-   Query the Aptos Mainnet API to fetch the account's APT coin balance.

## Features

-   Generate a new random wallet on each run.
-   Display the wallet's public address.
-   Fetch and display the wallet's APT balance from the Aptos mainnet.

## Dependencies

The project uses the following Rust crates:

-   `reqwest`: For making HTTP requests to the Aptos API.
-   `serde` & `serde_json`: For serializing and deserializing JSON data.
-   `ed25519-dalek`: For Ed25519 key generation and signing.
-   `rand`: For generating random data for key creation.
-   `sha3`: For hashing the public key to create the account address.
-   `hex`: For encoding the address hash into a hexadecimal string.

## How to Run

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd wallet
    ```

2.  **Build the project:**
    ```bash
    cargo build
    ```

3.  **Run the application:**
    ```bash
    cargo run
    ```

Upon running, the application will print a newly generated Aptos wallet address and its corresponding APT balance.

## How It Works

1.  **Wallet Generation**: The `generate_wallet()` function in `src/wallet.rs` uses the `ed25519-dalek` and `rand` crates to create a new cryptographic signing key.
2.  **Address Derivation**: The `get_address()` method takes the public part of the signing key, hashes it using SHA3-256, and then hex-encodes the hash to produce the final `0x` prefixed account address.
3.  **Balance Fetching**: The `get_balance()` function in `src/api.rs` constructs a URL for the Aptos mainnet API and makes a GET request to fetch the resources associated with the generated address. It then filters for the `0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>` resource and extracts the `value` field to display the balance.
