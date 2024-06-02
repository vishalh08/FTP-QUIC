## FTP-QUIC

# Overview

This project is all about a FTP using QUIC made with Rust.

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

# Extra Credits

- Used systems programming language (Rust)
- Uploaded the source code on GitHub
- Video demo of the FTP over QUIC protocol ([Watch Here](https://youtu.be/1BJ5SHsx2hk))
- Handing multiple clients by creating a new task for each accepted connection.
- Included a learning summary
