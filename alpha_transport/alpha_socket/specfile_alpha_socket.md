# **AlphaSocket Requirements Document**

## **1. Overview**
**AlphaSocket** is a high-performance, lock-free **IPC socket** designed to replace Unix sockets with sub-microsecond latency. It utilizes **[`disruptor-rs`](https://github.com/nicholassm/disruptor-rs)**, a Rust implementation of the Disruptor pattern, to ensure efficient, low-latency, and zero-copy inter-process communication (IPC).


Relevant crates:

- [`disruptor-rs`](https://github.com/nicholassm/disruptor-rs): A Rust implementation of the Disruptor pattern.
- [`memmap2`](https://github.com/RazrFalcon/memmap2-rs): A cross-platform Rust API for memory mapped buffers.

## **2. Objectives**
- Provide an **ultra-low latency** transport mechanism with single-digit nanosecond messaging latency.
- Serve as a **drop-in replacement** for Unix sockets with minimal API changes.
- Ensure **100% lock-free** operation by leveraging a pre-allocated ring buffer.
- Support **high throughput** (millions of messages per second).
- Avoid **syscalls** where possible to reduce context switching.
- Ensure **safe and robust** memory management while working directly with `mmap`.

## **3. Design Considerations**

### **3.1 Lock-Free Architecture**
- **Disruptor-based Ring Buffer:**
    - Implements **single-producer, single-consumer** messaging.
    - Eliminates the need for locks by using **sequence barriers** and busy-spin waiting.
    - Ensures cache-friendly, wait-free data access patterns.

### **3.2 Configurable Memory Buffer**
- **Memory-mapped files (`mmap`)**:
    - Pre-allocated **configurable buffer size** per connection.
    - The buffer size **must be a power of 2** (e.g., 16KB, 32KB, 64KB, etc.).
    - Avoids heap allocations for consistent performance.

### **3.3 Direct CPU Optimizations**
- **CPU Affinity & NUMA Awareness:**
    - Pin producer and consumer threads to dedicated cores.
    - Optimize memory locality for minimum cross-core latency.
- **Prefetching & Cache Optimization:**
    - Use **manual cache prefetching** (`_mm_prefetch()`) to reduce cache misses.
    - Align memory access to CPU cache lines (64-byte alignment).

## **4. Key Safety Concerns & Solutions**

### **4.1 Undefined Behavior & Segmentation Faults**
- **Ensured by Rust’s Ownership Model**
    - No **dangling pointers** since all references are scoped.
    - No **use-after-free** since `mmap` memory is managed safely.
- **Memory Safety in `mmap`**
    - Use `mmap` with **MAP_SHARED** and ensure proper page alignment.
    - Properly handle **page faults** to avoid undefined behavior.
- **Disruptor’s Sequence Barrier**
    - Prevents out-of-order memory accesses.

### **4.2 Deadlocks & Race Conditions**
- **Lock-free via Disruptor**
    - No mutexes, no RWLocks, no atomic contention.
- **Single Producer / Single Consumer Model**
    - Ensures deterministic execution without concurrency bugs.

### **4.3 Data Corruption & Message Ordering**
- **Sequence-based Integrity**
    - Messages are assigned **monotonic sequence numbers** for ordering.
- **Explicit Memory Fences**
    - Uses **compiler & CPU fences** to enforce ordering guarantees.

## **5. Performance Goals**

- **Latency (P50):** <50 ns
- **Latency (P99):** <150 ns
- **Throughput:** >50 million messages per second
- **Max Message Size:** Configurable, must be a power of 2
- **Transport Type:** Memory-mapped IPC  

### **6.1 Unix Stream API (POSIX-style)**
- `socket(AF_UNIX, SOCK_STREAM, 0)` - Create a Unix domain socket.
- `bind()` - Bind the socket to a file path.
- `listen()` - Set up a socket to accept connections.
- `accept()` - Accept incoming connections.
- `connect()` - Connect to a Unix domain socket.
- `send()` / `recv()` - Send and receive messages.
- `sendmsg()` / `recvmsg()` - Send and receive messages with ancillary data.
- `shutdown()` - Gracefully close one or both directions of the socket.
- `close()` - Close the socket.  


## **7. Usage Example: Echo Server & Client with AlphaSocket**

The following example demonstrates how to implement an **Echo Server** and a corresponding **Client** using AlphaSocket, mimicking the behavior of Unix sockets.

### **7.1 Echo Server (AlphaSocket-based)**
```rust
use alphasocket::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = UnixListener::bind("/tmp/alphasocket.sock").expect("Failed to bind socket");

    println!("AlphaSocket Echo Server is running...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut buffer = [0u8; 1024];
                    while let Ok(size) = stream.read(&mut buffer) {
                        if size == 0 {
                            break;
                        }
                        stream.write_all(&buffer[..size]).expect("Failed to echo back");
                    }
                });
            }
            Err(err) => eprintln!("Connection failed: {}", err),
        }
    }
}
```

### **7.2 Echo Client (AlphaSocket-based)**

```rust
use alphasocket::UnixStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = UnixStream::connect("/tmp/alphasocket.sock").expect("Failed to connect");

    let message = b"Hello, AlphaSocket!";
    stream.write_all(message).expect("Failed to send message");

    let mut buffer = [0u8; 1024];
    let size = stream.read(&mut buffer).expect("Failed to receive response");

    println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
}
```

### 7.3 Running the Example

Run the example using 

`cargo run --example echo_alpha_socket_server`,

then start a new terminal and run 

`cargo run --example echo_alpha_socket_client`.

## **8. High-Level Task List**

### **8.1 Core AlphaSocket Implementation**
- Implement `AlphaSocket` with a `mmap`-backed **configurable** ring buffer.
- Implement `read()` / `write()` APIs with sequence-based data integrity.
- Ensure **zero-copy** buffer access to avoid memory overhead.

### **8.2 Unix Socket Compatibility**
- Implement `UnixListener` core API (bind, listen, accept).
- Implement `UnixStream` core API (connect, send, receive, shutdown).
- Implement `sendmsg` / `recvmsg` support for ancillary data.
- Implement `shutdown` and `close` to gracefully close one or both directions of the socket.

### **8.3 Performance & Safety Enhancements**
- Optimize for **busy-spin polling** (zero syscalls).
- Implement **manual cache prefetching** for consistent low latency.
- Add **memory fences** to ensure correct read/write ordering.

### **8.4 Benchmarking & Validation**
- Implement a **latency benchmark** with P50, P90, P99, P999, min, mean, and max latency stats.
- Verify AlphaSocket’s performance on **macOS (M3 Max) and Linux (AMD)**.
- Ensure stability under **high concurrency (multi-client tests)**.  


