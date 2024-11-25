# Sliding Window Implementation

A high-performance sliding window implementation in Rust, featuring both safe and unsafe variants with array and vector-based storage options.

## Features

- Four distinct implementations to suit different needs:
  - Safe Array Storage
  - Safe Vector Storage
  - Unsafe Array Storage (with SIMD optimizations)
  - Unsafe Vector Storage

- Cache-line aligned storage for optimal performance
- Configurable window size and capacity
- Zero-cost abstractions
- Comprehensive test suite
- Detailed benchmarks

## Implementation Details

### Safe Array Storage
- Fixed-size array implementation
- Thread-safe and memory-safe
- Best for scenarios where safety is paramount
- No runtime bounds checking overhead

### Safe Vector Storage
- Dynamic size implementation
- Automatic memory management
- Ideal for varying window sizes
- Built on Rust's standard Vec type

### Unsafe Array Storage
- High-performance implementation
- Cache-line aligned (64-byte)
- SIMD optimizations for 4+ byte types
- 16-byte chunk copying for optimal memory throughput
- Best for performance-critical applications

### Unsafe Vector Storage
- Dynamic size with unsafe optimizations
- Manual memory management
- Combines flexibility with performance
- Suitable for advanced use cases

## Performance Benchmarks

| Operation | Safe Array | Unsafe Array | Improvement |
|-----------|------------|--------------|-------------|
| Push Single | 1.9 ns | 592.64 ps | -69.13% |
| Push with Rewind | 1.8 ns | 595.65 ps | -68.63% |
| Sequential Ops | 2.0 ns | 1.429 ns | -29.69% |
| Memory Access | 19 ns | 8.77 ns | -53.91% |

### Batch Operations Performance

| Batch Size | Safe Array | Unsafe Array | Improvement |
|------------|------------|--------------|-------------|
| 10 items | 19.8 ns | 6.61 ns | -66.74% |
| 50 items | 96.9 ns | 30.04 ns | -69.02% |
| 100 items | 189.7 ns | 64.21 ns | -66.14% |

## Usage

Enable the unsafe feature in your `Cargo.toml`:
```toml
[dependencies]
sliding_window = { version = "0.1.0", features = ["unsafe"] }
```

Basic usage example:
```rust
use sliding_window::{WindowStorage, UnsafeArrayStorage};

// Create a window of size 4 with capacity 8
let mut window: UnsafeArrayStorage<i32, 4, 8> = UnsafeArrayStorage::new();

// Push some values
window.push(100);
window.push(200);
window.push(300);
window.push(400);

// Get the window contents
assert_eq!(window.get_slice(), &[100, 200, 300, 400]);

// Push more values (window slides)
window.push(500);
window.push(600);
assert_eq!(window.get_slice(), &[300, 400, 500, 600]);
```

## Safety Notes

The unsafe implementations provide significant performance benefits but require careful usage:
- Ensure proper alignment for SIMD operations
- Be mindful of type sizes for optimal performance
- Use safe implementations when safety is more critical than performance

## Performance Optimization Details

The unsafe array implementation includes several optimizations:
1. Cache-line alignment (64-byte) for optimal memory access
2. SIMD-friendly memory operations for 4+ byte types
3. 16-byte chunk copying for efficient memory throughput
4. Type-size-aware optimizations
5. Minimal bounds checking in critical paths

## License

MIT License - See LICENSE file for details
