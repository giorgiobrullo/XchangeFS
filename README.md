# XchangeFS

XchangeFS is a decentralized peer-to-peer storage system designed to provide secure, efficient, and scalable file storage and sharing.

## Features

- **Decentralized Storage**: Distributes files across multiple nodes to ensure redundancy and availability.
- **Peer-to-Peer Network**: Directly connects users to share and retrieve files without a central server.
- **Security**: Implements encryption to protect data integrity and privacy.
- **Scalability**: Easily scales with the addition of new nodes to the network.
- **Efficiency**: Optimizes data transfer and storage to minimize latency and maximize throughput.

## How It Works

XchangeFS allows users to "exchange" disk space on their local system for space on other systems in the network. This decentralized approach ensures that storage is distributed and redundant, enhancing both availability and reliability.

## Prerequisites

Before installing XchangeFS, ensure you have the following prerequisites:

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).

## Building from Source

To build XchangeFS from source, follow these steps:

1. Ensure all prerequisites are installed.
2. Clone the repository:
        ```sh
        git clone https://github.com/yourusername/XchangeFS.git
        ```
3. Navigate to the project directory:
        ```sh
        cd XchangeFS
        ```
4. Build the project:
        ```sh
        cargo build --release
        ```

## Running Tests

To run tests for XchangeFS, use the following command:
```sh
cargo test
```