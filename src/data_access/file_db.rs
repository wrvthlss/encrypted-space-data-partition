use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::hash::Hash;
use std::io::{self, Write};

const FILE_PATH: &str = "users.txt";

// Read all usernames from the file and returns a HashSet.
pub fn read_username() -> io::Result<HashSet<String>> {
    let mut usernames = HashSet::new();

    if let Ok(contents) = fs::read_to_string(FILE_PATH) {
        for line in contents.lines() {
            usernames.insert(line.to_string());
        }
    }

    Ok(usernames)
}

// Write a new username to the file.
pub fn write_usernames(username: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)?;

    writeln!(file, "{}", username)?;
    Ok(())
}