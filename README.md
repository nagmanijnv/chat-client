# Rust TCP Chat Client

A simple **asynchronous TCP chat client** written in Rust using **Tokio**.  
Connects to a TCP chat server, reads messages from the server, and sends user input in real-time.

---

## Features

- Asynchronous client using **Tokio**
- Separate tokio tasks for reading from server and writing to server
- Non-blocking user input
- Command-line configurable server port

---

## Build & Run Commands
Commands to run and build the tcp chat client

### Build Using:
```bash
cargo build
```

### Run Using: 
```bash
cargo run <port>
```
