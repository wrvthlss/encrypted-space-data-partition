mod data_access;
mod business_logic;
mod presentation;

use business_logic::registration::register_new_username;
use presentation::session::Session;
use presentation::menu::display_menu;
use business_logic::messages::process_message;

use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

type Db = Arc<RwLock<HashMap<String, String>>>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9809").await?;
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    println!("[+] Server listening on 0.0.0.0:9809");

    loop {
        let (socket, addr) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            handle_connection(socket, addr, db).await;
        });
    }
}


async fn handle_connection(mut socket: TcpStream, addr: std::net::SocketAddr, db: Db) {
    println!("[*] Client {} connected", addr);

    // Step 1: Show menu
    if let Err(e) = display_menu(&mut socket).await {
        println!("[-] Failed to display menu to {}: {}", addr, e);
        return;
    }

    // Step 2: Read menu selection
    let mut buffer = [0; 1024];
    let selection = match socket.read(&mut buffer).await {
        Ok(n) => {
            if n == 0 {
                println!("[*] {} disconnected before selecting a menu option.", addr);
                return;
            }
            String::from_utf8_lossy(&buffer[..n]).trim().to_string()
        }
        Err(e) => {
            println!("[-] Failed to read menu selection from {}: {}", addr, e);
            return;
        }
    };

    match selection.as_str() {
        "1" => {
            // Placeholder for existing username logic
            if let Err(e) = socket.write_all(b"[!] Login with an existing username is not implemented yet.\n").await {
                println!("[-] Failed to send message to {}: {}", addr, e);
            }
            return; // Exit after displaying the message
        }
        "2" => {
            // Register a new username
            let username = match register_new_username(&mut socket, &addr).await {
                Some(username) => username,
                None => {
                    println!("[*] Client {} disconnected during registration.", addr);
                    return;
                }
            };

            // Create a session
            let session = Session::new_registered(username.clone());
            println!("[*] User '{}' logged in with session: {:?}", username, session);
        }
        "3" => {
            // Placeholder for guest logic
            if let Err(e) = socket.write_all(b"[!] Guest account logic is not implemented yet.\n").await {
                println!("[-] Failed to send message to {}: {}", addr, e);
            }
            return; // Exit after displaying the message
        }
        "4" => {
            // Disconnect
            if let Err(e) = socket.write_all(b"Good-bye...\n").await {
                println!("[-] Failed to send QUIT acknowledgment to {}: {}", addr, e);
            }
            println!("[*] {} disconnected via QUIT.", addr);
            return; // Exit
        }
        _ => {
            // Invalid selection
            if let Err(e) = socket.write_all(b"[!] Invalid selection. Please enter 1, 2, 3, or 4.\n").await {
                println!("[-] Failed to send invalid selection message to {}: {}", addr, e);
            }
            return; // Exit after invalid input
        }
    }

    // Step 3: Command handling loop
    loop {
        match socket.read(&mut buffer).await {
            Ok(n) => {
                if n == 0 {
                    println!("[*] {} disconnected.", addr);
                    return; // Client disconnected
                }

                let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

                // Handle special commands like QUIT
                if input.eq_ignore_ascii_case("QUIT") {
                    if let Err(e) = socket.write_all(b"Good-bye...\n").await {
                        println!("[-] Failed to send QUIT acknowledgment to {}: {}", addr, e);
                    }
                    println!("[*] {} disconnected via QUIT.", addr);
                    return; // Exit the loop
                }

                // Process other commands like SET, GET, SHOW
                let response = process_message(&input, db.clone()).await;

                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    println!("[-] Failed to send response to {}: {}", addr, e);
                    return; // Exit on write failure
                }
            }
            Err(e) => {
                println!("[-] Failed to read from socket for {}: {}", addr, e);
                return; // Exit on read failure
            }
        }
    }
}