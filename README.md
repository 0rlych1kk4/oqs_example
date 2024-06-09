# OQS Example

This project demonstrates the usage of post-quantum cryptography (PQC) in secure key management and communication.

## Overview

In the era of quantum computing, traditional cryptographic algorithms like RSA and ECC are vulnerable to attacks from quantum computers. Post-quantum cryptography (PQC) provides alternative cryptographic algorithms that are resistant to attacks from both classical and quantum computers.

This project showcases how to use PQC, specifically the Kyber key encapsulation mechanism (KEM), for secure key management and communication.

## Features

- **Key Management**: Generates key pairs and encapsulates shared secrets using Kyber KEM.
- **Communication**: Allows secure communication between two parties by encapsulating and decapsulating shared secrets.

## Modules

### 1. Key Management

This module handles the generation of key pairs and the encapsulation of shared secrets using the Kyber KEM.

### 2. Communication

This module facilitates secure communication between two parties. It encapsulates and decapsulates shared secrets using the Kyber KEM.

## Usage

1. Clone the repository:

    ```bash
    git clone https://github.com/0rlych1kk4/oqs_example.git
    ```

2. Navigate to the project directory:

    ```bash
    cd oqs_example
    ```

3. Build the project:

    ```bash
    cargo build
    ```

4. Run the project:

    ```bash
    cargo run
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

