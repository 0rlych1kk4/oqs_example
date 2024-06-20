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

# MIT License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

Copyright (c) 2024 0rlych1kk4

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

