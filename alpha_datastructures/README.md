# Quant Engine Data Structures

## Overview

The `queng_datastructures` module provides high-performance, specialized data structures optimized for quantitative
trading applications. It includes implementations of ring buffers (based on the LMAX Disruptor pattern) and sliding
windows, designed for efficient market data processing and real-time analytics.

## Components

### Ring Buffer Implementation

A high-performance ring buffer based on the LMAX Disruptor pattern, optimized for low-latency event processing.

#### Core Components:

1. **Barrier System**
    - Processing sequence barrier
    - Event sequence management
    - Thread coordination

2. **Consumer Architecture**
    - Batch event processor
    - Event handling system
    - Consumer coordination

3. **Producer Implementation**
    - Multi-producer support
    - Single-producer optimization
    - Producer-consumer coordination

4. **Ring Buffer Core**
    - Constant array-based implementation
    - Memory-efficient design
    - Lock-free operations

5. **Sequence Management**
    - Atomic sequence handling
    - Sequence coordination
    - Thread-safe operations

6. **Wait Strategies**
    - Blocking wait strategy
    - Spinlock wait strategy
    - Custom wait strategy support

7. **Execution Control**
    - Thread pool executor
    - Task scheduling
    - Resource management

8. **DSL and Builder**
    - Rust-specific Disruptor builder
    - Fluent API
    - Configuration management

### Sliding Window Implementation

Efficient sliding window data structure with both safe and unsafe implementations for different performance
requirements.

#### Features:

1. **Storage Types**
    - Array-based storage
    - Vector-based storage
    - Optimized memory management

2. **Safe Implementation**
    - `storage_safe::storage_array`: Safe array-based implementation
    - `storage_safe::storage_vec`: Safe vector-based implementation
    - Bounds checking and memory safety

3. **Unsafe Implementation**
    - `storage_unsafe::unsafe_storage_array`: High-performance array implementation
    - `storage_unsafe::unsafe_storage_vec`: High-performance vector implementation
    - Zero-cost abstractions

## Usage

### Ring Buffer Example

```rust
use ring_buffer::{RustDisruptorBuilder, EventHandler};

// Create a ring buffer
let ring_buffer = RustDisruptorBuilder::new()
    .with_size(1024)
    .with_producer_type(ProducerType::Single)
    .with_wait_strategy(WaitStrategy::Blocking)
    .build();

// Add event handlers
ring_buffer.handle_events(MyEventHandler::new());

// Start processing
ring_buffer.start();
```

### Sliding Window Example

```rust
use sliding_window::{StorageArray, StorageVec};

// Create a sliding window with array storage
let mut window = StorageArray::new(100);

// Add data
window.push(data);

// Process window
for item in window.iter() {
    // Process item
}
```

## Design Principles

1. **Performance**
    - Lock-free operations where possible
    - Memory efficiency
    - Cache-friendly data structures
    - Zero-copy operations

2. **Safety**
    - Thread-safe implementations
    - Memory safety guarantees
    - Safe and unsafe variants for different needs
    - Comprehensive error handling

3. **Flexibility**
    - Multiple producer/consumer patterns
    - Configurable wait strategies
    - Customizable event handling
    - Adaptable to different use cases

4. **Reliability**
    - Consistent performance
    - Predictable behavior
    - Error recovery mechanisms
    - Resource cleanup

## Development

### Prerequisites

- Rust 1.84.0 or higher
- Cargo build system
- Understanding of concurrent programming

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Benchmarking

```bash
cargo bench
```

## Performance Considerations

1. **Ring Buffer**
    - Optimized for high-throughput scenarios
    - Minimal contention between producers and consumers
    - Efficient memory reuse
    - Cache-line aware design

2. **Sliding Window**
    - Efficient memory management
    - Optimized for sequential access
    - Minimal reallocation
    - Fast insertion and removal

## Documentation

Generate documentation using:

```bash
cargo doc --no-deps --open
```
