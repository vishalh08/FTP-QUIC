use color_eyre::eyre::Result;
use s2n_quic::Server;
use std::{path::Path};
use std::net::ToSocketAddrs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use bincode;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncReadExt;
use md5;

#[derive(Debug)]
struct ServerOptions{
  address: String,
  port: u16,
  cert: String,
  key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    HandshakeInitiation = 0x01,
    HandshakeResponse = 0x02,
    Request = 0x03,
    Response = 0x04,
    Data = 0x05,
    DownloadRequest(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct PDU {
    msg_type: MessageType,
    length: u32,
    sequence_number: u32,
    checksum: String,
    data: Vec<u8>,
    filename: String,
}

impl PDU {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        bincode::serialize(self)
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<bincode::ErrorKind>> {
        bincode::deserialize(&bytes)
    }
}

#[tokio::main]
async fn run(options: ServerOptions) -> Result<()>  {

  let host_port_string = format!("{}:{}",
    options.address, options.port).to_socket_addrs()?.next().unwrap();
  let mut server = Server::builder()
        .with_tls((Path::new(&options.cert), Path::new(&options.key)))?
        .with_io(host_port_string)?
        .start()?;
  println!("{:#?} In server...", options);
  while let Some(mut connection) = server.accept().await {
    println!("Accepted a new connection");
    tokio::spawn(async move {
        while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
            println!("Accepted a new bidirectional stream");
            tokio::spawn(async move {
                loop {
                    match stream.receive().await {
                        Ok(Some(rdata)) => {
                            let pdu: PDU = PDU::from_bytes(rdata.to_vec()).unwrap();
                            println!("Received PDU: {:?}", pdu);
                            match pdu.msg_type {
                                MessageType::Data => {
                                    let calculated_checksum = format!("{:x}", md5::compute(&pdu.data));
                                    if pdu.checksum == calculated_checksum {
                                        let mut file = File::create(&pdu.filename).await.unwrap();
                                        file.write_all(&pdu.data).await.unwrap();
                                        println!("Received a valid file from the client and wrote it to {}", pdu.filename);

                                        // Send a response back to the client
                                        let response_pdu = PDU {
                                            msg_type: MessageType::Response,
                                            length: 0,
                                            sequence_number: 0,
                                            checksum: String::new(),
                                            data: vec![],
                                            filename: String::new(),
                                        };
                                        let response_pdu_bytes = response_pdu.to_bytes().unwrap();
                                        if let Err(e) = stream.send(response_pdu_bytes.into()).await {
                                            eprintln!("An error occurred while sending response: {}", e);
                                            return Err(e.into());
                                        }
                                        println!("Sent a response to the client");
                                    } else {
                                        println!("Received a file from the client, but the data is corrupted");
                                    }
                                },
                                MessageType::DownloadRequest(filename) => {
                                    let mut file = File::open(filename.clone()).await?;
                                    let mut buffer = Vec::new();
                                    file.read_to_end(&mut buffer).await?;

                                    let pdu = PDU {
                                        msg_type: MessageType::Data,
                                        length: buffer.len() as u32,
                                        sequence_number: 0,
                                        checksum: format!("{:x}", md5::compute(&buffer)),
                                        data: buffer,
                                        filename: filename.to_string(),
                                    };
                                    println!("Sending PDU: {:?}", pdu);
                                    let pdu_bytes = pdu.to_bytes().unwrap();
                                    if let Err(e) = stream.send(pdu_bytes.into()).await {
                                        eprintln!("An error occurred while sending data: {}", e);
                                        return Err(e.into());
                                    }
                                    println!("Sent a file to the client");

                                    // Send a response back to the client
                                    let response_pdu = PDU {
                                        msg_type: MessageType::Response,
                                        length: 0,
                                        sequence_number: 0,
                                        checksum: String::new(),
                                        data: vec![],
                                        filename: String::new(),
                                    };
                                    let response_pdu_bytes = response_pdu.to_bytes().unwrap();
                                    if let Err(e) = stream.send(response_pdu_bytes.into()).await {
                                        eprintln!("An error occurred while sending response: {}", e);
                                        return Err(e.into());
                                    }
                                    println!("Sent a response to the client");
                                },
                                _ => {
                                    println!("Received an unsupported message type");
                                    break;
                                },
                            }
                        },
                        Ok(None) => {
                            println!("The stream has been closed by the client");
                            break;
                        }
                        Err(e) => {
                            eprintln!("Connection Closed  {}", e);
                            break;
                        }
                    }
                }
                Ok::<(), std::io::Error>(())
            });
        }
    });
  }
  Ok(())
}

const DEFAULT_PORT: u16 = 54321;

pub fn do_server(address: String, cert:String, key:String) -> Result<()> {
  println!("Starting server...");
  println!("Listening on {address} using port {DEFAULT_PORT}...");

  let options = ServerOptions {
    address,
    port: DEFAULT_PORT,
    cert,
    key,
  };

  run(options)?;

  Ok(())
}
