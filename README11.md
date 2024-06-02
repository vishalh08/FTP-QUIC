## FTP-QUIC

# Overview

This project is all about a FTP using QUIC made with Rust.

# Features

Adopts a multi-client single-server architecture.
Employs QUIC for efficient text transmission.
Ensures secure communication through the use of TLS encryption certificates.
Verifies the integrity of files using checksums.
Facilitates both uploading and downloading of files.

# Requirements

Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install OpenSSL

sudo apt update
sudo apt install openssl

# Certificates

The certs directory already includes the keys and certificates that I’ve generated for both the client and server.

# Running the Code

Steps

git clone https://github.com/vishalh08/FTP-QUIC.git

cd FTP-QUIC

run server from the FTP-QUIC directoy where the git directory is cloned

cargo run -- server --cert ./certs/server.crt --key ./certs/server.key

run client from the FTP-QUIC directoy where the git directory is cloned(use seperate terminal)

cargo run --bin quicrs -- client --address 127.0.0.1 --port 54321 --cert ./certs/ca.cert

Extra Credits

https://youtu.be/1BJ5SHsx2hk
