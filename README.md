# Rust TCP Server

A lightweight, multithreaded TCP server built with Rust's standard library. It accepts client connections, reads incoming data, logs messages, and responds with `Hello there!`.

## Features

- TCP communication using `std::net`.
- Thread-per-connection concurrency.
- Modular and organized project structure.
- No external dependencies.

## Configuration

| Setting | Default |
|---|---|
| IP Address | `127.0.0.1` |
| Port | `8080` |
| Buffer Size | `1024` bytes |
| Response | `Hello there!` |
| Protocol | TCP |

The server listens on the local loopback interface and accepts connections from the same machine.

## Project Structure

```text
src/
├── main.rs          # Application entry point
├── config.rs        # Server configuration
├── constants.rs     # Shared constants
├── utils.rs         # Utility functions
└── server/
    ├── mod.rs       # Listener and connection management
    └── client.rs    # Client communication
```

## Getting Started

**Run the server:**

```bash
cargo run
```

**Test with Netcat** in another terminal:

```bash
nc 127.0.0.1 8080
```

Type a message and press Enter. The server responds with:

```text
Hello there!
```

**Verify the listening port:**

```bash
ss -ltnp | grep ':8080'
```

## How It Works

1. The server binds to the configured address and listens for connections.
2. Each accepted connection is handled in a separate thread.
3. The server reads and logs the received data.
4. It sends the predefined response and finishes handling the connection.

## Limitations

- Reads only once per connection.
- Sends one response per connection.
- Uses an unbounded thread-per-connection model.
- Implements raw TCP, not HTTP.