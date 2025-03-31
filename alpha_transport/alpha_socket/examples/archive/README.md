# Alpha Socket Archive Examples

This directory contains a set of examples that demonstrate the archiving capabilities of the `alpha_socket` library, which is an ultra-low latency IPC (Inter-Process Communication) mechanism based on memory-mapped ring buffers.

## Overview

The examples implement a financial market data distribution system with three components:

1. **Tick Server**: Generates and publishes market data ticks
2. **Tick Client**: Consumes market data ticks in real-time
3. **Tick Archiver**: Captures all data for post-processing or compliance purposes

This architecture demonstrates the "drop copy" pattern, where all communication between the server and client is automatically mirrored to an archiver without impacting performance.

## Components

### Tick Server (`tick_server/main.rs`)

The server component:
- Creates two UNIX domain sockets: one for client communication and one for the archiver
- Generates simulated tick data for Apple stock (AAPL) with increasing price and volume
- Sends 15 data points at 500 microsecond intervals
- Uses `AlphaSocketWithArchiver` to automatically mirror all messages to both client and archiver

### Tick Client (`tick_client/main.rs`)

The client component:
- Connects to the server socket
- Deserializes incoming JSON tick data
- Displays price, symbol, and volume information
- Terminates after receiving 15 tick data points

### Tick Archiver (`tick_archiver/main.rs`)

The archiver component:
- Connects to a dedicated archiver socket
- Passively receives all messages sent between server and client
- Logs each message to the console. In a real-world scenario, this will be used for post-processing or regulatory compliance purposes

## Data Format

The examples use the following data structure for market ticks:

```rust
struct TickData {
    symbol: String,    // Stock symbol (e.g., "AAPL")
    timestamp: u64,    // UNIX timestamp
    price: f64,        // Current price
    volume: u64,       // Trading volume
}
```

## Running the Examples

To run these examples, start each component in a separate terminal:

1. First, start the archiver:
   ```bash
   cargo run --example tick_archiver
   ```

2. Next, start the server:
   ```bash
   cargo run --example tick_server
   ```

3. Finally, start the client:
   ```bash
   cargo run --example tick_client
   ```

The server will wait for both the archiver and client to connect before sending data.

## How It Works

These examples showcase the following features of the `alpha_socket` library:

1. **Zero-Copy IPC**: Using memory-mapped ring buffers for high performance
2. **Drop Copy Pattern**: Automatically mirroring data to an archiver without additional code
3. **Synchronized Communication**: Coordinated startup sequence ensuring all components are ready
4. **Structured Data Exchange**: Using JSON serialization for tick data transfer

## Notes

- The socket files are created at `/tmp/alpha_socket` and `/tmp/alpha_archiver`
- Old socket files are automatically removed on server startup
- The examples use a simple line-based protocol with JSON-serialized data
