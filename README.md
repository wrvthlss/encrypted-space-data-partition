# **Encrypted Space and Data Partition**
[![Rust](https://img.shields.io/badge/Language-Rust-blue)](https://www.rust-lang.org/)  
An application which will generate encrypted ephemeral environments and tooling for confidential information sharing.
---

## **Current Version: 0.0.1c**
The current version includes a fully refactored codebase with modular architecture:
- **N-tier design** for separation of concerns.
- Functional menu system for user registration and login.
- Database-backed username persistence.
- Basic CRUD operations (`SET`, `GET`, `SHOW`) on a key-value store.

---

## **Features by Version**

### **Version 0.0.1a**
Initial implementation:
- Basic server setup using **Tokio**.
- Simple client-server communication.
- In-memory key-value store.

### **Version 0.0.1b**
Enhancements:
- Menu system for interacting with the server.
- User registration feature:
    - Registers new users and persists them to a file-based database (`users.txt`).

### **Version 0.0.1c**
Refactored architecture:
- Modular codebase split into:
    - **Data Access Layer**: Handles database operations.
    - **Business Logic Layer**: Manages user registration, login, and command processing.
    - **Presentation Layer**: Handles client communication (e.g., menus, session management).
- Fully operational database commands:
    - `SET <key> <value>`: Adds or updates a key-value pair.
    - `GET <key>`: Retrieves the value for a key.
    - `SHOW`: Displays all key-value pairs.
- User login system (`Option 1`): Enables existing users to log in.

---

## **Upcoming Version: 0.1.0**
Planned features:
- Fully functional menu system.
- Guest account functionality.
- Robust error handling and logging.
- Tests for all modules (unit and integration).

---

## **Usage**

### **Running the Server**
1. Build the Docker image:
   ```bash
   docker build -t encrypted-space-data-partition .

2. Run the container:
    ```bash
    docker run -p 9809:9809 encrypted-space-data-partition

## **Connecting as a Client**
1. User `netcat` or similar to connect to server:
    ```bash
   nc 127.0.0.1 9809

2. Follow the menu options to:
- Register or log in as a user.
- Perform database operations (e.g., SET, GET, SHOW).
- Disconnect with QUIT.

---

## **Architecture**
The project adopts a **modular, N-tier architecture:**

**Data Access Layer:**
- Handles all interactions with the file-based database (`users.txt`).
- Encapsulated in `data_access/`.

**Business Logic Layer:**
- Implements user registration, login, and command processing.
- Encapsulated in `business_logic/`.

**Presentation Layer:**
- Manages client interactions, menus, and session state.
- Encapsulated in `presentation/`.

---

## **Contributing**
Contributions are welcome! Feel free to:
- Submit issues for bugs or feature requests.
- Open pull requests with new features or improvements.

---

## **License**
This project is licensed under the MIT License. See the LICENSE file for details.