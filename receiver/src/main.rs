use openssl::symm::{Cipher, decrypt};
use tokio::net::{TcpListener, UdpSocket};
use std::fs::File;
use std::io::Write;
use tokio::io::AsyncReadExt;

async fn decrypt_data(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key, None, data).expect("Decryption failed")
}

async fn receive_tcp(port: u16, output_file: &str, key: &[u8]) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    let (mut socket, _) = listener.accept().await?;
    let mut buffer = vec![0u8; 1024];

    let mut file = File::create(output_file)?;
    while let Ok(size) = socket.read(&mut buffer).await {
        if size == 0 {
            break;
        }
        let decrypted = decrypt_data(&buffer[..size], key).await;
        file.write_all(&decrypted)?;
    }

    Ok(())
}

async fn receive_udp(port: u16, output_file: &str, key: &[u8]) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", port)).await?;
    let mut buffer = vec![0u8; 1024];

    let mut file = File::create(output_file)?;
    let (size, _) = socket.recv_from(&mut buffer).await?;
    let decrypted = decrypt_data(&buffer[..size], key).await;
    file.write_all(&decrypted)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: <tcp|udp> <port> <output_file>");
        return;
    }

    let protocol = &args[1];
    let port: u16 = args[2].parse().unwrap();
    let output_file = &args[3];

    // Replace this with your AES key
    let aes_key = b"your_aes_key_16b";

    if protocol == "tcp" {
        receive_tcp(port, output_file, aes_key).await.unwrap();
    } else {
        receive_udp(port, output_file, aes_key).await.unwrap();
    }
}

