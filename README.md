# blockchain-validator-node
A simple blockchain validator node in Rust for learning how networks work.

## What it does

This is a basic implementation of a blockchain validator node in Rust. It's designed to help understand how a simple network can be built and how nodes can communicate with each other. The node can validate transactions and add them to the blockchain.

## Features

* `validator`: the core logic for validating transactions and adding them to the blockchain
* `network`: a simple network implementation that allows nodes to communicate with each other
* `encryption`: basic encryption and decryption using a simple symmetric key cipher

## Installation

To install the project, run the following commands:
```bash
cargo install --path .
cargo run
```
This will build and run the node. You can then use the `cargo add_transaction` command to add transactions to the blockchain.

## Build from source

To build the project from source, run the following command:
```bash
cargo build
```
This will build the project in release mode.

## Testing

To run the tests, run the following command:
```bash
cargo test
```
The project has a test suite that covers the core logic of the validator node.

## Project Structure

* `src/validator.rs`: the core logic for validating transactions and adding them to the blockchain
* `src/network.rs`: a simple network implementation that allows nodes to communicate with each other
* `src/encryption.rs`: basic encryption and decryption using a simple symmetric key cipher
* `tests/validator.rs`: tests for the validator logic
* `Cargo.toml`: the project configuration file
* `README.md`: this file
* `LICENSE`: the project license

## License

Copyright (c) 2026 SamyAlderson

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.