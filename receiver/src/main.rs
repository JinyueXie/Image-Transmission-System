use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tokio::net::{TcpListener, UdpSocket};
use tokio::io::AsyncReadExt;  // Add this for read operations
use openssl::symm::{decrypt, Cipher};

// Function to decrypt data using OpenSSL
fn decrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cipher = Cipher::aes_128_ecb();
    Ok(decrypt(cipher, key, None, data)?)
}

async fn receive_tcp(port: u16, output_file: &str, key: &[u8]) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("Listening on TCP port {}", port);

    let (mut socket, _) = listener.accept().await?;
    println!("Client connected");

    // Receive file size first
    let mut size_buf = [0u8; 8];
    socket.read_exact(&mut size_buf).await?;
    let expected_size = u64::from_ne_bytes(size_buf);
    println!("Expecting {} bytes of data", expected_size);

    // Prepare output file and buffer
    let mut file = File::create(output_file)?;
    let mut received_data = Vec::with_capacity(expected_size as usize);
    let mut buffer = vec![0u8; 1024];

    // Receive data in chunks
    while received_data.len() < expected_size as usize {
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        received_data.extend_from_slice(&buffer[..n]);
        println!("Received {} of {} bytes", received_data.len(), expected_size);
    }

    // Decrypt and write to file
    println!("Decrypting data...");
    match decrypt_data(&received_data, key) {
        Ok(decrypted_data) => {
            file.write_all(&decrypted_data)?;
            println!("Data written to {}", output_file);
        }
        Err(e) => eprintln!("Decryption error: {}", e),
    }

    Ok(())
}

async fn receive_udp(port: u16, output_file: &str, key: &[u8]) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", port)).await?;
    println!("Listening on UDP port {}", port);

    // Receive total size first
    let mut size_buf = [0u8; 8];
    let (_, _) = socket.recv_from(&mut size_buf).await?;
    let expected_size = u64::from_ne_bytes(size_buf);
    println!("Expecting {} bytes of data", expected_size);

    // Prepare buffer for receiving chunks
    let mut chunks: HashMap<u64, Vec<u8>> = HashMap::new();
    let mut buffer = vec![0u8; 1024 + 8]; // Extra 8 bytes for chunk number
    let mut received_bytes = 0;

    loop {
        let (n, _) = socket.recv_from(&mut buffer).await?;
        if n >= 8 {
            let chunk_number = u64::from_ne_bytes(buffer[..8].try_into().unwrap());
           
            // Check for end marker
            if chunk_number == u64::MAX {
                println!("Received end marker");
                break;
            }

            // Store chunk data
            chunks.insert(chunk_number, buffer[8..n].to_vec());
            received_bytes += n - 8;
           
            println!("Received chunk {}, total bytes: {}/{}",
                    chunk_number, received_bytes, expected_size);
        }
    }

    // Combine chunks in order
    let mut combined_data = Vec::with_capacity(expected_size as usize);
    for i in 0..chunks.len() as u64 {
        if let Some(chunk) = chunks.get(&i) {
            combined_data.extend_from_slice(chunk);
        } else {
            eprintln!("Missing chunk {}", i);
            return Ok(());
        }
    }

    // Decrypt and write to file
    println!("Decrypting data...");
    match decrypt_data(&combined_data, key) {
        Ok(decrypted_data) => {
            let mut file = File::create(output_file)?;
            file.write_all(&decrypted_data)?;
            println!("Data written to {}", output_file);
        }
        Err(e) => eprintln!("Decryption error: {}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <tcp|udp> <port> <output_file>", args[0]);
        return Ok(());
    }

    let protocol = &args[1];
    let port: u16 = args[2].parse()?;
    let output_file = &args[3];

    // AES key (must match sender's key)
    let aes_key: [u8; 16] = [
        0x2D, 0x52, 0xA8, 0xF6, 0xA9, 0x25, 0x19, 0xFA,
        0x80, 0xF2, 0xFE, 0xAF, 0x09, 0x9B, 0xBF, 0x01
    ];

    match protocol.as_str() {
        "tcp" => receive_tcp(port, output_file, &aes_key).await?,
        "udp" => receive_udp(port, output_file, &aes_key).await?,
        _ => eprintln!("Unsupported protocol: {}", protocol),
    }

    Ok(())
}
