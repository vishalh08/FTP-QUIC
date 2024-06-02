## FTP-QUIC

# Overview

This project is all about a FTP using QUIC made with Rust.

# Requirements

Install Rust (https://www.youtube.com/watch?v=enk0o7eWNsc), you can follow the video for installation process.
Install OpenSSL (https://slproweb.com/products/Win32OpenSSL.html)

##Setup
clone the repository
git clone https://github.com/vishalh08/FTP-QUIC.git

Generate the following,

Generate CA key
openssl genrsa -out ca.key 4096

Generate CA certificate
openssl req -new -x509 -key ca.key -sha256 -days 365 -out ca.cert -subj "/C=US/ST=PHL/O=Drexel-University"

Generate the server key
openssl genrsa -out server.key 4096

Generate the CSR (Certificate Signing Request)
openssl req -new -key server.key -out server.csr -config certificate.conf

Generate the server certificate signed by the CA
openssl x509 -req -in server.csr -CA ca.cert -CAkey ca.key -CAcreateserial -out server.crt -days 365 -sha256 -extfile certificate.conf -extensions req_ext

# Running the Code

command to run server
cargo run -- server --cert ./certs/server.crt --key ./certs/server.key

command to run client
cargo run --bin quicrs -- client --address 127.0.0.1 --cert ./certs/ca.cert
