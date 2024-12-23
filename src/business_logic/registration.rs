use std::fmt::format;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt, split};
use std::net::SocketAddr;

use crate::data_access::file_db;

pub async fn register_new_username(socket: &mut TcpStream, add: &SocketAddr) -> Option<String> {
    let (reader, mut writer) = split(socket);
    let mut reader = BufReader::new(reader);
    let mut input = String::new();

    loop {
        // Prompt for new username.
        if let Err(e) = writer.write_all(b"[!] Please enter a new username:\n").await {
            println!("[-] Failed to send new username prompt to {}: {}", add, e);
            return None;
        }

        // Read the client's input.
        input.clear();
        match reader.read_line(&mut input).await {
            Ok(0) => {
                    println!("[-] {} disconnected during new username registration", add);
                    return None;
                }
            Ok(_) => {
                let username = input.trim().to_string();

                // Validate username.
                if username.is_empty() {
                    if let Err(e) = writer.write_all(b"[!] Username cannot be empty...\n").await {
                        println!("[-] Failed to send empty username message to {}: {}", add, e);
                        return None;
                    }
                    continue;
                }

                // Check if username exists in the database.
                let usernames= file_db::read_username().unwrap_or_default();
                if usernames.contains(&username) {
                    if let Err(e) = writer
                        .write_all(b"[!] Username already exist, please select another...\n")
                        .await
                    {
                        println!("[-] Failed to send username exists message to: {}", add);
                        return None;
                    }
                    continue; // Re-prompt.
                }

                // Add the username to the database.
                if let Err(e) = file_db::write_usernames(&username) {
                    println!("[-] Failed to write new username to file for {}: {}", add, e);
                    return None;
                }

                // ACK successful registration.
                if let Err(e) = writer
                    .write_all(
                        format!("[*] Welcome, {}! You are now registered...\n", username)
                            .as_bytes(),
                    )
                    .await
                    {
                        println!("[-] Failed to acknowledge new registration from {}: {}", add, e);
                        return None; // Exit.
                    }
                return Some(username);
            }
            Err(e) => {
                println!("[-] Failed to read new username from {}: {}", add, e);
                return None; // Exit.
            }
        }
    }
}

pub async fn login_existing_username(socket: &mut TcpStream, addr: &SocketAddr) -> Option<String> {
    let (reader, mut writer) = split(socket);
    let mut reader = BufReader::new(reader);
    let mut input = String::new();

    loop {
        // Prompt the user to enter an existing username.
        if let Err(e) = writer.write_all(b"[!] Please enter your username:\n").await {
            println!("[-] Failed to send username prompt to {}: {}", addr, e);
            return None;
        }

        // Read the username from the client.
        input.clear();
        match reader.read_line(&mut input).await {
            Ok(n) => {
                if n == 0 {
                    println!("[-] {} disconnected before entering a username.", addr);
                    return None; // Disconnected.
                }

                let username = input.trim().to_lowercase();
                println!("[DEBUG] Client entered username: {}", username);

                // Validation.
                let usernames = file_db::read_username().unwrap_or_default();
                println!("[DEBUG] Usernames in database: {:?}", usernames);

                if usernames.contains(&username) {
                    // Username exists.
                    if let Err(e) = writer.write_all(
                        format!("[*] Welcome back, {}!\n", username).as_bytes()
                    ).await {
                        println!("[-] Failed to send welcome message to: {}: {}", addr, e);
                        return None; // Exit on failure.
                    }
                    return Some(username); // Logged in.
                } else {
                    // Username does not exist: re-prompt.
                    if let Err(e) = writer.write_all(
                        b"[!] Username not found, try again...\n"
                    ).await {
                        println!("[-] Failed to send username not found message to: {}: {}", addr, e);
                        return None; // Exit on failure.
                    }
                }
            }
            Err(e) => {
                println!("[-] Failed to read username input from {}: {}", addr, e);
                return None; // Exit on failure.
            }
        }
    }
}