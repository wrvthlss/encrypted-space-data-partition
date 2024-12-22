use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type Db = Arc<RwLock<HashMap<String, String>>>;

pub async fn process_message(input: &str, db: Db) -> String {
    match input {
        command if command.starts_with("SET") => {
            let parts: Vec<&str> = command.split_whitespace().collect();

            if parts.len() == 3 {
                let mut db_write = db.write().await;
                db_write.insert(parts[1].to_string(), parts[2].to_uppercase());
                format!("[+] Key {} set successfully\n", parts[1])
            } else {
                "[-] Invalid SET command\n".to_string()
            }
        }
        command if command.starts_with("GET") => {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() == 2 {
                let db_read = db.read().await;
                match db_read.get(parts[1]) {
                    Some(value) => format!("Value: {}\n", value),
                    None => "[!] Key not found\n".to_string(),
                }
            } else {
                "[-] Invalid GET command\n".to_string()
            }
        }
        command if command.eq_ignore_ascii_case("SHOW") => {
            let db_read = db.read().await;

            if db_read.is_empty() {
                "[!] Database is empty!\n".to_string()
            } else {
                let mut output = String::from("[*] Database contents:\n");
                for (key, value) in db_read.iter() {
                    output.push_str(&format!("{} {}\n", key, value));
                }
                output
            }
        }
        _ => "[!] Unknown command\n".to_string(),
    }
}
