# Quant Engine Import Core

## Overview

The `queng_import/core` module provides core import functionality for the Quant Engine project. It manages the import of
various data types including service configurations, metadata, integration configurations, and portfolio configurations
through specialized managers.

## Components

### Service Import (`service_import`)

Manages the import of service configurations and specifications.

#### Features:

- Service configuration import
- PostgreSQL SMDB integration
- Environment-aware configuration
- Debug and test modes

### Metadata Import (`metadata_import`)

Handles the import of market and trading metadata.

#### Features:

- Market metadata import
- PostgreSQL MDDB integration
- Proxy auto-detection
- Debug and test modes

### Integration Import (`integration_import`)

Manages external system integration configurations.

#### Features:

- Integration configuration import
- System connectivity setup
- Environment-specific configurations
- Debug support

### Config Import (`config_import`)

Handles portfolio and trading configurations.

#### Features:

- Portfolio configuration import
- Trading parameter setup
- Environment-aware settings
- Debug capabilities

## Usage

### Service Import

```rust
use service_import::ServiceImportManager;

// Create default manager
let manager = ServiceImportManager::new().await;

// Create manager with debug mode
let debug_manager = ServiceImportManager::with_debug().await;

// Create manager with test and debug mode
let test_manager = ServiceImportManager::with_test_and_debug().await;
```

### Metadata Import

```rust
use metadata_import::MetadataImportManager;

// Create default manager
let manager = MetadataImportManager::new().await;

// Create manager with debug mode
let debug_manager = MetadataImportManager::with_debug().await;

// Create manager with test and debug mode
let test_manager = MetadataImportManager::with_test_and_debug().await;
```

## Environment Integration

### Local Development

```bash
# Required environment variables
ENV=LOCAL
PG_USER=<username>
PG_PASSWORD=<password>
PG_DATABASE=<database>
```

### CI Environment

```bash
# Required environment variables
ENV=CI
PG_USER=<username>
PG_PASSWORD=<password>
PG_DATABASE=<database>
```

### Cluster Environment

```bash
# Required environment variables
ENV=CLUSTER
PG_USER=<username>
PG_PASSWORD=<password>
PG_DATABASE=<database>
DNS_SERVER=<cluster_dns>
```

## Database Integration

### PostgreSQL Integration

- SMDB (Service Management Database)
- MDDB (Market Data Database)
- Environment-specific connections
- Connection pooling

## Design Principles

1. **Environment Awareness**
    - Automatic environment detection
    - Environment-specific configurations
    - Debug mode support

2. **Modularity**
    - Specialized import managers
    - Clear component separation
    - Focused responsibilities

3. **Reliability**
    - Comprehensive error handling
    - Database connection management
    - Import validation

4. **Flexibility**
    - Multiple import modes
    - Test environment support
    - Debug capabilities

## Development

### Prerequisites

- Rust 1.84.0 or higher
- PostgreSQL database
- Environment variables set
- Network access for imports

### Testing

```bash
cargo test
```

### Debug Mode

Enable debug mode for detailed import information:

```rust
let manager = ServiceImportManager::with_debug().await;
```

## Error Handling

The components handle several error conditions:

- Database connection failures
- Import validation errors
- Configuration parsing errors
- Environment setup issues

## Performance Considerations

1. **Database Operations**
    - Connection pooling
    - Efficient queries
    - Batch operations

2. **Import Processing**
    - Optimized data handling
    - Memory-efficient operations
    - Parallel processing where possible

3. **Resource Management**
    - Connection cleanup
    - Memory management
    - Resource pooling

## Security

1. **Database Security**
    - Secure connection handling
    - Credential management
    - Access control

2. **Configuration Security**
    - Environment variable protection
    - Secure configuration storage
    - Access validation

3. **Import Security**
    - Data validation
    - Source verification
    - Secure transfer protocols
