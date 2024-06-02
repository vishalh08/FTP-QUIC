use color_eyre::eyre::Result;
use s2n_quic::{client::Connect, Client};
use std::{path::Path, net::SocketAddr};
use std::net::ToSocketAddrs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use bincode;
use serde::{Serialize, Deserialize};
use std::io::{stdin, stdout, Write};
use md5;

#[derive(Debug)]
struct ClientOptions{
  address: String,
  port: u16,
  cert: String,
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
    checksum: String, // Change this line
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
async fn run(options:ClientOptions) -> Result<()> {
    let host_port_string = format!("{}:{}",
      options.address, options.port).to_socket_addrs()?.next().unwrap();

    let addr: SocketAddr = "0.0.0.0:0".parse()?;
    let client = Client::builder()
        .with_tls(Path::new(&options.cert))?
        .with_io(addr)?
        .start()?;

    println!("Connecting client...");
    let connect = Connect::new(host_port_string).with_server_name("localhost");
    let mut connection = client.connect(connect).await?;
    println!("Client connected...");

    // ensure the connection doesn't time out with inactivity
    connection.keep_alive(true)?;

    // open a new stream and split the receiving and sending sides
    let stream = connection.open_bidirectional_stream().await?;
    let (mut receive_stream, mut send_stream) = stream.split();

    //YOUR APPLICATION PROTOCOL STARTS HERE

    let mut sequence_number = 0;

    loop {
        println!("Please select an option:");
        println!("1. Upload a file");
        println!("2. Download a file");
        println!("3. Exit");
        print!("Enter your choice: ");
        stdout().flush().unwrap();
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        let choice: u8 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                //STEP 1: Send a file to the server
                print!("Enter the name of the file to upload: ");
                stdout().flush().unwrap();
                let mut filename = String::new();
                stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim();
                let mut file = File::open(filename).await?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).await.unwrap();

                let pdu = PDU {
                    msg_type: MessageType::Data,
                    length: buffer.len() as u32,
                    sequence_number: sequence_number,
                    checksum: format!("{:x}", md5::compute(&buffer)), // Change this line
                    data: buffer,
                    filename: filename.to_string(),
                };
                println!("Sending PDU: {:?}", pdu);
                sequence_number += 1;
                let pdu_bytes = pdu.to_bytes().unwrap();
                send_stream.send(pdu_bytes.into()).await.expect("stream should be open");
                println!("Sent a file to the server");

                match receive_stream.receive().await {
                    Ok(Some(data)) => {
                        let pdu: PDU = PDU::from_bytes(data.to_vec()).unwrap();
                        match pdu.msg_type {
                            MessageType::Response => {
                                println!("Received a response from the server");
                                
                            },
                            _ => println!("Received an unexpected message type"),
                        }
                    },
                    Ok(None) => {
                        println!("The stream has been closed by the server");
                        
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("An error occurred while receiving data: {}", e);
                        return Err(e.into());
                    }
                }
            },
            2 => {
                //STEP 2: Send a download request to the server
                print!("Enter the name of the file to download: ");
                stdout().flush().unwrap();
                let mut filename = String::new();
                stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim();
                let pdu = PDU {
                    msg_type: MessageType::DownloadRequest(filename.to_string()),
                    length: 0,
                    sequence_number: sequence_number,
                    checksum: String::new(), // Change this line
                    data: vec![],
                    filename: String::new(), // Add this line
                };
                println!("Sending PDU: {:?}", pdu);
                sequence_number += 1;
                let pdu_bytes = pdu.to_bytes().unwrap();
                send_stream.send(pdu_bytes.into()).await.expect("stream should be open");
                println!("Sent a download request to the server");

                //STEP 3: Receive a file from the server
                let rdata = match receive_stream.receive().await {
                    Ok(Some(data)) => data,
                    Ok(None) => {
                        println!("The stream has been closed by the server");
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("An error occurred while receiving data: {}", e);
                        return Err(e.into());
                    }
                };
                let pdu: PDU = PDU::from_bytes(rdata.to_vec()).unwrap();
                println!("Received PDU: {:?}", pdu); // This will now include the sequence number
                // Calculate the checksum of the received data
                let calculated_checksum = format!("{:x}", md5::compute(&pdu.data)); // Change this line

                // Compare it with the checksum in the PDU
                if pdu.checksum == calculated_checksum {
                    // If they match, write the data to a file
                    let mut file = File::create(&pdu.filename).await.unwrap(); // Use the filename from the PDU
                    file.write_all(&pdu.data).await.unwrap();
                    println!("Received a valid file from the server and wrote it to {}", pdu.filename);
                } else {
                    // If they don't match, the data is corrupted
                    println!("Received a file from the server, but the data is corrupted");
                }
                
            },
            3 => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 3.");
            },
        }
    }
    
    Ok(())
}


const DEFAULT_PORT: u16 = 54321; // This should be the same as the server's port

pub fn do_client(address: String, cert: String) -> Result<()> {
  println!("Starting client...");
  println!("Connecting to {address} on port {DEFAULT_PORT}...");

  let options = ClientOptions {
    address,
    port: DEFAULT_PORT,
    cert,
  };

  run(options)?;

  Ok(())
}

