# Pattern Manager

## Overview

The `pattern_manager` component provides a comprehensive pattern recognition and management system for the Quant Engine
project. It handles various trading patterns including base patterns, extra patterns, and directional (long/short)
patterns using OHLCV bar data.

## Core Features

### Pattern Types

- **Base Patterns**: Fundamental trading patterns
- **Extra Patterns**: Advanced trading patterns
- **Long Patterns**: Bullish trading patterns
- **Short Patterns**: Bearish trading patterns

### Pattern Management

- Pattern evaluation and validation
- Pattern length management
- Pattern updates with new data
- Thread-safe pattern access

### OHLCV Integration

- Works with OHLCV bar data
- Sliding window pattern analysis
- Real-time pattern updates
- Historical pattern analysis

## Usage

### Basic Usage

```rust
use pattern_manager::PatternManager;
use common_trade::PatternType;
use common_data_bar::OHLCVBar;

// Create pattern manager
let manager = PatternManager::new();

// Update patterns with new data
let window: [OHLCVBar; 6] = /* ... */;
manager.update_patterns(&PatternType::Base, &window)?;

// Get evaluation results
let result = manager.get_eval_result(&PatternType::Base, index)?;

// Get pattern length
let len = manager.get_pattern_len(&PatternType::Base)?;
```

### Pattern Type Selection

```rust
use common_trade::PatternType;

// Available pattern types
let base_pattern = PatternType::Base;    // Fundamental patterns
let extra_pattern = PatternType::Extra;  // Advanced patterns
let long_pattern = PatternType::Long;    // Bullish patterns
let short_pattern = PatternType::Short;  // Bearish patterns
```

## Components

### Pattern Trait (`abstract_trait.rs`)

- Common interface for all patterns
- Pattern evaluation methods
- Pattern length management
- Pattern update functionality

### Base Pattern (`base_pattern.rs`)

- Fundamental pattern implementation
- 43 distinct pattern types
- OHLCV-based calculations
- Pattern validation

### Extra Pattern (`extra_pattern.rs`)

- Advanced pattern recognition
- Complex pattern combinations
- Extended pattern analysis
- Custom pattern definitions

### Directional Patterns

#### Long Pattern (`long_pattern.rs`)

- Bullish pattern recognition
- Uptrend identification
- Buy signal generation
- Trend strength analysis

#### Short Pattern (`short_pattern.rs`)

- Bearish pattern recognition
- Downtrend identification
- Sell signal generation
- Trend weakness analysis

### Fields (`fields.rs`)

- Common pattern constants
- Threshold definitions
- Pattern parameters
- Calculation utilities

## Design Principles

1. **Thread Safety**
    - RefCell-based pattern access
    - Safe concurrent pattern updates
    - Protected pattern state

2. **Flexibility**
    - Multiple pattern types
    - Extensible pattern system
    - Customizable parameters

3. **Performance**
    - Efficient pattern updates
    - Fast pattern evaluation
    - Minimal memory usage

4. **Reliability**
    - Comprehensive error handling
    - Pattern validation
    - State consistency

## Development

### Prerequisites

- Rust 1.84.0 or higher
- `common_data_bar` crate
- `common_trade` crate

### Testing

```bash
cargo test
```

### Documentation

Generate documentation using:

```bash
cargo doc --no-deps --open
```

## Pattern Implementation

### Adding New Patterns

1. Create a new pattern struct
2. Implement the `PatternTrait`
3. Add pattern to `PatternManager`
4. Add pattern type to `PatternType`

### Pattern Validation

```rust
impl PatternTrait for CustomPattern {
    fn get_eval_result(&self, index: usize) -> Result<bool, String> {
        if index >= self.pattern_len() {
            return Err(format!("Index out of bounds: {}", index));
        }
        Ok(self.patterns[index])
    }
}
```

## Performance Considerations

1. **Memory Usage**
    - Fixed-size pattern arrays
    - Efficient OHLCV processing
    - Minimal state storage

2. **Computation**
    - Optimized pattern calculations
    - Efficient window updates
    - Fast pattern matching

3. **Thread Safety**
    - Lock-free pattern access
    - Minimal contention
    - Efficient RefCell usage

## Error Handling

The component handles several error conditions:

- Invalid pattern indices
- Pattern update failures
- Invalid OHLCV data
- Pattern validation errors
