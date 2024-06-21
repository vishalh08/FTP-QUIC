## FTP-QUIC

# Overview

FTP-QUIC is a project that implements a File Transfer Protocol (FTP) using the QUIC protocol. It is developed using the Rust programming language.

# Features

- Adopts a multi-client single-server architecture.
- Employs QUIC for efficient text transmission.
- Ensures secure communication through the use of TLS encryption certificates.
- Verifies the integrity of files using checksums.
- Facilitates both uploading and downloading of files.

# Requirements

Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install OpenSSL

```bash
sudo apt update
sudo apt install openssl
```

# Certificates

The certs directory already includes the keys and certificates that I’ve generated for both the client and server.

# Running the Code

Steps

```bash
git clone https://github.com/vishalh08/FTP-QUIC.git

cd FTP-QUIC

# Run server from the FTP-QUIC directory where the git directory is cloned

cargo run -- server --cert ./certs/server.crt --key ./certs/server.key

# Run client from the FTP-QUIC directory where the git directory is cloned (use separate terminal)

cargo run --bin quicrs -- client --address 127.0.0.1 --port 54321 --cert ./certs/ca.cert
```

