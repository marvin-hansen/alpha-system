# Common Order Extensions

## Overview

The `common_order_ext` module provides Simple Binary Encoding (SBE) extensions for order management types in the Quant
Engine project. These extensions enable efficient binary serialization and deserialization of various order-related
messages.

## Components

### Order Creation Extensions (`order_create_ext`)

Extends the `OrderCreate` type with SBE encoding/decoding capabilities.

#### Features:

- Binary encoding of order creation messages
- Efficient decoding of incoming orders
- Comprehensive error handling
- Zero-copy deserialization where possible

### Order Cancellation Extensions

#### Single Order Cancel (`order_cancel_ext`)

- Binary encoding of single order cancellation
- Decoding of cancellation messages
- Error handling for cancel operations

#### Bulk Cancel (`order_cancel_all_ext`)

- Efficient Binary encoding of bulk cancellation requests
- Decoding of bulk cancel messages
- Optimized for multiple order cancellations

### Order Update Extensions (`order_update_ext`)

- Binary encoding of order modifications
- Decoding of update messages
- State transition handling

### Order ID Extensions

#### Client ID (`order_id_client_ext`)

- Client-side order ID management
- ID encoding and decoding
- Client-specific ID validation

#### Exchange ID (`order_id_exchange_ext`)

- Exchange order ID handling
- ID conversion utilities
- Exchange-specific ID validation

## Usage

### Order Creation Example

```rust
use common_order::OrderCreate;
use common_order_ext::SbeOrderCreateExtension;

// Create a new order
let order = OrderCreate::new(/* ... */);

// Encode to SBE binary format
let (size, buffer) = order.encode_to_sbe()?;

// Decode from SBE binary format
let decoded_order = OrderCreate::decode_from_sbe(&buffer)?;
```

### Bulk Cancel Example

```rust
use common_order::OrderCancelAll;
use common_order_ext::SbeOrderCancelAllExtension;

// Create bulk cancel request
let cancel_all = OrderCancelAll::new(/* ... */);

// Encode to SBE binary format
let (size, buffer) = cancel_all.encode_to_sbe()?;

// Decode from SBE binary format
let decoded_cancel = OrderCancelAll::decode_from_sbe(&buffer)?;
```

## Error Handling

The extensions provide two main error types:

- `SbeEncodeError`: Handles encoding failures
- `SbeDecodeError`: Handles decoding failures

## Design Principles

1. **Zero-Cost Abstractions**
    - Compile-time optimizations
    - No runtime overhead
    - Efficient memory usage

2. **Type Safety**
    - Strong compile-time guarantees
    - Clear error types
    - Safe binary parsing

3. **Performance**
    - Zero-copy operations where possible
    - Minimal allocations
    - Efficient binary encoding/decoding

4. **Reliability**
    - Comprehensive error handling
    - Validation at all stages
    - Clear error messages

## Integration with SBE

These extensions integrate with the `sbe_messages_order` crate to provide:

- Standard-compliant SBE encoding
- Efficient binary message formats
- Consistent serialization across the system

## Development

### Prerequisites

- Rust 1.84.0 or higher
- `common_order` crate
- `sbe_messages_order` crate
- `sbe_types` crate

### Testing

```bash
cargo test
```

### Documentation

Generate documentation using:

```bash
cargo doc --no-deps --open
```

## Performance Considerations

1. **Binary Format**
    - Compact message representation
    - Fast serialization/deserialization
    - Cache-friendly layout

2. **Memory Usage**
    - Minimal temporary allocations
    - Efficient buffer management
    - Zero-copy operations where possible

3. **Error Handling**
    - Fast error paths
    - Clear error messages
    - No panic conditions
