use std::fmt::format;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt, split};
use std::net::SocketAddr;

use crate::data_access::file_db;

pub async fn register_new_username(
    writer: &mut tokio::io::WriteHalf<TcpStream>,
    reader: &mut BufReader<tokio::io::ReadHalf<TcpStream>>,
    addr: &SocketAddr,
) -> Option<String> {
    let mut input = String::new();

    loop {
        if let Err(e) = writer.write_all(b"[!] Please enter a new username:\n").await {
            println!("[-] Failed to send new username prompt to {}: {}", addr, e);
            return None;
        }

        input.clear();
        match reader.read_line(&mut input).await {
            Ok(0) => {
                println!("[-] {} disconnected during new username registration.", addr);
                return None;
            }
            Ok(_) => {
                let username = input.trim().to_string();

                if username.is_empty() {
                    if let Err(e) = writer.write_all(b"[!] Username cannot be empty...\n").await {
                        println!("[-] Failed to send empty username message to {}: {}", addr, e);
                        return None;
                    }
                    continue;
                }

                let usernames = file_db::read_username().unwrap_or_default();
                if usernames.contains(&username) {
                    if let Err(e) = writer.write_all(b"[!] Username already exists, please select another...\n").await {
                        println!("[-] Failed to send username exists message to {}: {}", addr, e);
                        return None;
                    }
                    continue;
                }

                if let Err(e) = file_db::write_usernames(&username) {
                    println!("[-] Failed to write new username to file for {}: {}", addr, e);
                    return None;
                }

                if let Err(e) = writer.write_all(format!("[*] Welcome, {}! You are now registered...\n", username).as_bytes()).await {
                    println!("[-] Failed to acknowledge new registration for {}: {}", addr, e);
                    return None;
                }

                return Some(username);
            }
            Err(e) => {
                println!("[-] Failed to read new username input from {}: {}", addr, e);
                return None;
            }
        }
    }
}
pub async fn login_existing_username(
    writer: &mut tokio::io::WriteHalf<TcpStream>,
    reader: &mut BufReader<tokio::io::ReadHalf<TcpStream>>,
    addr: &SocketAddr,
) -> Option<String> {
    let mut input = String::new();

    loop {
        if let Err(e) = writer.write_all(b"[!] Please enter your username:\n").await {
            println!("[-] Failed to send username prompt to {}: {}", addr, e);
            return None;
        }

        input.clear();
        match reader.read_line(&mut input).await {
            Ok(0) => {
                println!("[-] {} disconnected before entering a username.", addr);
                return None;
            }
            Ok(_) => {
                let username = input.trim().to_lowercase();

                let usernames = file_db::read_username().unwrap_or_default();

                if usernames.contains(&username) {
                    if let Err(e) = writer.write_all(format!("[*] Welcome back, {}!\n", username).as_bytes()).await {
                        println!("[-] Failed to send welcome message to {}: {}", addr, e);
                        return None;
                    }
                    return Some(username);
                } else {
                    if let Err(e) = writer.write_all(b"[!] Username not found, try again...\n").await {
                        println!("[-] Failed to send username not found message to {}: {}", addr, e);
                        return None;
                    }
                }
            }
            Err(e) => {
                println!("[-] Failed to read username input from {}: {}", addr, e);
                return None;
            }
        }
    }
}
