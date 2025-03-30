# PostgreSQL Import Tools

## Overview

The `queng_import/postgres` module provides specialized import tools for PostgreSQL database integration in the Quant
Engine project. It handles various import operations for metadata, services, configurations, and integrations through
dedicated import managers.

## Components

### Metadata Import (`pg_import_metadata`)

Manages metadata import operations into PostgreSQL.

#### Features:

- Workflow determination
- Metadata import execution
- Progress tracking
- Performance monitoring

### Service Import (`pg_import_services`)

Handles service configuration imports.

#### Features:

- Service configuration import
- Import status checking
- Service counting
- Duplicate prevention

### Configuration Import (`pg_import_config`)

Manages configuration data imports.

#### Features:

- Portfolio configuration import
- Trading parameter import
- Configuration validation
- Version management

### Integration Import (`pg_import_integrations`)

Handles external system integration imports.

#### Features:

- Integration configuration import
- Connection management
- System synchronization
- Status tracking

### Print Utilities (`pg_import_print_utils`)

Provides shared printing utilities.

#### Features:

- Progress reporting
- Debug output
- Status headers
- Duration tracking

## Usage

### Metadata Import

```rust
use metadata_import::MetadataImportManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize manager
    let manager = MetadataImportManager::new().await;
    
    // Determine workflow
    let workflow = manager.determine_workflow(None).await?;
    
    // Execute workflow
    manager.execute_workflow(&workflow).await?;
    
    Ok(())
}
```

### Service Import

```rust
use service_import::ServiceImportManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize manager
    let manager = ServiceImportManager::new().await;
    
    // Check existing imports
    if !manager.check_if_already_imported().await {
        // Import services
        manager.import_services().await?;
    }
    
    Ok(())
}
```

## Performance Optimization

### Memory Management

- Uses MiMalloc allocator
- Efficient memory utilization
- Resource cleanup
- Memory pooling

### Import Performance

```rust
// Enable debug mode for performance monitoring
let manager = MetadataImportManager::with_debug().await;

// Track operation duration
let start = Instant::now();
manager.execute_workflow(&workflow).await?;
print_duration(DBG, "Execution took", &start.elapsed());
```

## Development

### Prerequisites

- Rust 1.84.0 or higher
- PostgreSQL database
- Tokio runtime
- MiMalloc allocator

### Environment Setup

```bash
# Required environment variables
PG_USER=<username>
PG_PASSWORD=<password>
PG_DATABASE=<database>
```

### Testing

```bash
cargo test
```

## Error Handling

The module handles several error conditions:

- Database connection failures
- Import validation errors
- Workflow execution failures
- Resource constraints

## Best Practices

1. **Import Operations**
    - Check existing data
    - Validate imports
    - Track progress
    - Handle errors

2. **Performance**
    - Use async operations
    - Monitor durations
    - Optimize memory
    - Batch operations

3. **Data Integrity**
    - Validate data
    - Prevent duplicates
    - Maintain consistency
    - Version control

## Database Integration

### Connection Management

- Connection pooling
- Timeout handling
- Retry mechanisms
- Error recovery

### Transaction Handling

- ACID compliance
- Rollback support
- Consistency checks
- Isolation levels

## Security

1. **Database Security**
    - Secure connections
    - Credential management
    - Access control
    - Audit logging

2. **Data Protection**
    - Input validation
    - Output sanitization
    - Error masking
    - Secure storage

## Monitoring

### Progress Tracking

- Import progress
- Operation duration
- Resource usage
- Error rates

### Status Reporting

- Import status
- Operation counts
- Success rates
- Performance metrics
