# blockchain-validator-node

A simple blockchain validator node implemented in Rust for educational purposes.

## What and Why

This project is a basic blockchain validator node implemented in Rust. It's designed to be a learning tool for those interested in blockchain technology and Rust programming. The node is responsible for validating transactions and maintaining a local copy of the blockchain.

## Install

To run the validator node, you'll need Rust installed on your system. You can download Rust from the official website. Once Rust is installed, you can clone this repository and run the following command to build and run the node:
```bash
cargo run
```
## Usage

To use the validator node, you'll need to create a new instance of the `Validator` struct and pass it a `Network` instance. The `Validator` struct is responsible for validating transactions and maintaining a local copy of the blockchain. The `Network` struct provides utilities for communicating with the network.

Here's an example of how to use the validator node:
```rust
use blockchain_validator_node::{Validator, Network};

fn main() {
    let network = Network::new();
    let validator = Validator::new(network);

    // Add some transactions to the validator
    validator.add_transaction(1, 2);
    validator.add_transaction(3, 4);

    // Validate the transactions
    validator.validate_transactions();
}
```
## Build from Source

This project uses Cargo as its build system, so you can build it from source by running the following command:
```bash
cargo build
```
This will compile the project and create an executable in the `target` directory.

## Project Structure

The project is structured as follows:
```markdown
blockchain-validator-node/
Cargo.toml
src/
main.rs
validator.rs
network.rs
utils.rs
tests/
test_validator.rs
test_network.rs
Cargo.lock
README.md
```
## License

This project is licensed under the MIT License.

## Features

This project includes the following features:

* Validator: The main component of the project, responsible for validating transactions and maintaining a local copy of the blockchain.
* Network: Provides utilities for communicating with the network.
* Encryption: Not currently implemented, but planned for future development.

## Dependencies

This project depends on the following crates:

* rand: For generating random numbers.
* serde: For serializing and deserializing data.
* serde_json: For serializing and deserializing JSON data.
* log: For logging errors and warnings.
* tokio: For asynchronous networking.

## Credits

This project was created by Samy Alderson.