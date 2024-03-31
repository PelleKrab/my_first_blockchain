# My First Blockchain

A simple blockchain implementation in Rust. This project is a proof of concept designed to explore the fundamentals of blockchain technology and Rust programming. It's intended for educational purposes and to demonstrate a basic blockchain operation including transaction processing, block mining, and chain validation.

## Features

- **Blockchain:** Core blockchain logic for maintaining a secure and immutable ledger.
- **Transactions:** Support for creating and validating transactions.
- **Block Mining:** Implements multithreaded mining of blocks with transaction data.
- **Chain Validation:** Ensures the integrity of the blockchain with chain and block validation techniques.

## Example Usage

Provide a simple example of how to use the blockchain, such as creating a transaction, adding it to a block, and appending it to the blockchain. For example, the `src/main.rs` file outlines a basic scenario of generating key pairs, creating transactions, mining a block, and validating the blockchain and transactions.

## Contributing

Contributions are welcome! If you have ideas for improvements or want to add new features, please feel free to submit a pull request or open an issue.

## Future Work

- Integration of a peer-to-peer network using libp2p.
- Implementation of account and validator logic.
- Exploration of state checks, gas mechanisms, and various consensus algorithms.

## Website
Visit <a href="http://pellekrab.com" target="_blank">pellekrab.com</a>.

## License

MIT License

Copyright (c) 2024 Pelle Krabbenhoeft

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
