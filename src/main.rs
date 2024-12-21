use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};

type Db = Arc<RwLock<HashMap<String, String>>>;

// Version 5
// Helper functions to read all usernames from the username file.
fn read_username(file_path: &str) -> io::Result<HashSet<String>> {
    let mut usernames = HashSet::new();

    if let Ok(contents) = fs::read_to_string(file_path) {
        for line in contents.lines() {
            usernames.insert(line.to_string());
        }
    }

    Ok(usernames)
}

// Version 5
// Helper function to write a username to the file
fn write_usernames(file_path: &str, username: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", username)?;
    Ok(())
}

async fn register_new_username(socket: &mut TcpStream, addr: &SocketAddr) -> Option<String> {
    const FILE_PATH: &str = "users.txt";
    let mut buffer = [0; 1024];

    loop {
        // Prompt for a new username.
        if let Err(e) = socket.write_all(b"[*] Please enter a new username:\n").await {
            println!("[-] Failed to send new username prompt to {}: {}", addr, e);
            // Exit on failure.
            return None;
        }

        // Read the client's input.
        match socket.read(&mut buffer).await {
            Ok(n) => {

                if n == 0 {
                    println!("[!] {} disconnected during new username registration...", addr);
                    return None; // Client disconnected.
                }

                let username = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

                // Validate the input
                if username.is_empty() {
                    if let Err(e) = socket.write_all(b"[!] Username cannot be empty. Please try again..\n").await {
                        println!("[-] Failed to send empty username message to {}: {}", addr, e);
                        return None; // Exit on failure.
                    }
                    continue; // Re-prompt the user
                }

                // Check if the username exists in the database.
                let usernames = read_username(FILE_PATH).unwrap_or_default();
                if usernames.contains(&username) {
                    if let Err(e) = socket.write_all(b"[!] Username already exists. Please try a different one..\n").await {
                        println!("[-] Failed to send username message to {}: {}", addr, e);
                        return None;
                    }
                    continue; // Re-prompt user.
                }

                // Add the new username to the database.
                if let Err(e) = write_usernames(FILE_PATH, &username){
                    println!("[-] Failed to write new username to file for {}: {}", addr, e);
                    return None;
                }

                // Acknowledge successfull registration.
                if let Err(e) = socket.write_all(format!("[*] Welcome, {}! You are now registered...\n",username).as_bytes()).await {
                    println!("[-] Failed to acknowledge new registration for {}: {}", addr, e);
                    return None; // Exit on failure.
                }

                return Some(username);

            }
            Err(e) => {
                println!("[-] Failed to read new username from {}: {}", addr, e);
                return None; // Exit on failure.
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9809").await?;
    let db : Db = Arc::new(RwLock::new(HashMap::new()));

    println!("[+] Server listening on port 9809");

    loop {
        let (socket, addr) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            handle_connection(socket, addr, db).await;
        });
    }
}

// Version 4 Refactor
// async fn capture_username(socket: &mut TcpStream, addr: &SocketAddr) -> Option<String> { // Version 6

// Version 6 -- New signature to support returning Pinned heap memory for recursive future call.
async fn capture_username(socket: &mut TcpStream, addr: &SocketAddr) -> Option<String> {
    let mut buffer = [0; 1024];

    loop {
        // Step 1: Display the menu to the client
        let menu = "\
            Please select a following option:\n\
            1. Enter existing username.\n\
            2. Enter new username to register.\n\
            3. Select this option to be assigned a read-only guest account and be assigned a random username.\n\
            4. Disconnect from the server.\n\n\
            Please enter 1, 2, 3 or 4:\n";

        if let Err(e) = socket.write_all(menu.as_bytes()).await {
            println!("[-] Failed to send menu to {}: {}", addr, e);
            return None; // Exit on failure
        }

        // Step 2: Read the user's selection
        match socket.read(&mut buffer).await {
            Ok(n) => {
                if n == 0 {
                    println!("[*] {} disconnected before making a selection.", addr);
                    return None; // Client disconnected
                }

                let selection = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                println!("[*] Received menu selection from {}: {}", addr, selection);

                // Step 3: Validate the selection
                match selection.as_str() {
                    "1" => {
                        // Placeholder for existing username logic (to be implemented later)
                        if let Err(e) = socket
                            .write_all(b"[!] Entering existing username logic (not yet implemented).\n")
                            .await
                        {
                            println!(
                                "[-] Failed to send placeholder message for option 1 to {}: {}",
                                addr, e
                            );
                        }
                    }
                    "2" => {
                        // Register a new username
                        return register_new_username(socket, addr).await;
                    }
                    "3" => {
                        // Placeholder for guest logic (to be implemented later)
                        if let Err(e) = socket
                            .write_all(b"[!] Guest account logic not yet implemented.\n")
                            .await
                        {
                            println!(
                                "[-] Failed to send placeholder message for option 3 to {}: {}",
                                addr, e
                            );
                        }
                    },
                    "4" => {
                        // Client wants to disconnect
                        if let Err(e) = socket.write_all(b"[!] Disconnecting from the server.\n").await {
                            println!("[-] Failed to send disconnect message to {}: {}", addr, e);
                        }
                        println!("[*] {} chose to disconnect.", addr);
                        return None; // Disconnect
                    }
                    _ => {
                        // Invalid input: notify the client and continue the loop
                        if let Err(e) = socket.write_all(b"[!] Invalid selection. Please enter 1, 2, 3, or 4:\n").await {
                            println!(
                                "[-] Failed to send invalid selection message to {}: {}",
                                addr, e
                            );
                            return None; // Exit on failure
                        }
                    }
                }
            }
            Err(e) => {
                println!("[-] Failed to read menu selection from {}: {}", addr, e);
                return None; // Exit on failure
            }
        }
    }
}
async fn handle_connection(mut socket: TcpStream, addr: SocketAddr, db: Db) {
    println!("[*] Client {} connected", addr.to_string());

    // Version 4: Get username // Refactor
    let username = match capture_username(&mut socket, &addr).await {
        Some(username) => username,
        None => return,
    };

    let mut buffer = [0; 1024];
    loop { // Version 2
        match socket.read(&mut buffer).await {
            // Version 4 check if username was entered.
            Ok(n) => {
                if n == 0 {
                    println!("[*] {} disconnected before providing a username...", addr.to_string());
                    return;
                }

                // let input = String::from_utf8_lossy(&buffer[..n]);
                let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

                // Version 2
                if input.eq_ignore_ascii_case("QUIT") {
                    // Disconnect if the client sends QUIT.
                    let _ = socket.write_all(b"Good-bye...\n").await;

                    // Version 2.5
                    // Explicitly shut down the connection
                    if let Err(e) = socket.shutdown().await {
                        println!("[-] Failed to shutdown socket: {}", e);
                    }

                    return;
                }

                // let response = process_message(input.trim(), db).await;
                let response = process_message(&input, db.clone()).await;

                // Version 2
                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    println!("[-] Failed to write to socket: {}", e);
                    return;
                }
                //socket.write_all(response.as_bytes()).await.unwrap();
            }
            Err(e) => {
                println!("[-] Failed to read from socket: {}", e);
                return;
            }
        }
    }
}

async fn process_message(input: &str, db: Db) -> String {
    // Version 3
    //let mut db_write = db.write().await;

    match input {
        command if command.starts_with("SET") => {
            let parts: Vec<&str> = command.split_whitespace().collect();

            if parts.len() == 3 {
                // Write lock for SET.
                let mut db_write = db.write().await; // Version 3

                db_write.insert(parts[1].to_string(), parts[2].to_uppercase());
                format!("[+] Key {} set successfully\n", parts[1])
            } else {
                "[-] Invalid SET command\n".to_string()
            }
        }
        command if command.starts_with("GET") => {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() == 2 {
                // Read lock for GET
                let db_read = db.read().await; // Version 3

                match db_read.get(parts[1]) {
                    Some(value) => format!("Value: {}\n", value),
                    None => "[!] Key not found\n".to_string(),
                }
            } else {
                "[-] Invalid GET command\n".to_string()
            }
        }
        // Version 3
        command if command.eq_ignore_ascii_case("SHOW") => {
            // Read lock for SHOW.
            let db_read = db.read().await;

            if db_read.is_empty() {
                // Version 3 -- add \n
                "[!] Database is empty!\n".to_string()
            } else {
                // Format key-value pairs.
                let mut output = String::from("[*] Database contents:\n");
                for (key, value) in db_read.iter() {
                    output.push_str(&format!("{} {}\n", key, value));
                }
                output
            }
        }
        _ => "[!] Unknown command\n".to_string()
    }
}
