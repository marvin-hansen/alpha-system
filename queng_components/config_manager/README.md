# Configuration Manager

## Overview

The `config_manager` component provides a robust configuration management system for the Quant Engine project. It
handles service configuration, environment detection, DNS resolution, and service discovery across different deployment
contexts.

## Core Features

### Environment Management

- Automatic environment detection (LOCAL, CI, CLUSTER)
- Environment-specific configuration adaptation
- Debug mode for configuration diagnostics

### Service Configuration

- Service host and port management
- Service dependency resolution
- Socket address configuration
- Environment-specific service adaptation

### DNS Resolution

- Internal (cluster) DNS resolution
- External DNS resolution
- DNS server configuration
- Hostname resolution

## Usage

### Basic Configuration

```rust
use config_manager::CfgManager;
use common_config::{ServiceID, ServiceConfig};

// Create default configuration
let cfg = CfgManager::default();

// Create configuration with debug mode
let cfg_debug = CfgManager::default_with_debug();

// Create custom configuration
let cfg_custom = CfgManager::new(
    ServiceID::Custom,
    custom_service_config()
);
```

### Service Discovery

```rust
use config_manager::CfgManager;

async fn get_service_endpoints(cfg: &CfgManager) -> Result<(), InitError> {
    // Get SMDB service endpoint
    let (host, port) = cfg.get_smdb_host_port().await?;
    
    // Get CMDB service endpoint
    let (host, port) = cfg.get_cmdb_host_port().await?;
    
    // Get service dependencies
    let deps = cfg.get_service_dependencies();
    Ok(())
}
```

## Environment Configuration

### Local Development

Required environment variables:

```bash
ENV=LOCAL
```

### CI Environment

Required environment variables:

```bash
ENV=CI
```

### Cluster Environment

Required environment variables:

```bash
ENV=CLUSTER
DNS_SERVER=<cluster_dns>
PG_USER=<postgres_user>
PG_PASSWORD=<postgres_password>
PG_DATABASE=<postgres_database>
```

## Components

### Configuration Service (`cfg_svc`)

- Service host/port management
- Service dependency resolution
- Environment-specific configuration

### DNS Management (`dns`, `dns_resolve`)

- DNS server configuration
- Hostname resolution
- Internal/external DNS handling

### Environment Management (`env`)

- Environment type detection
- Environment variable handling
- Context-specific configuration

### Health Check (`cfg_svc_health_check`)

- Service health monitoring
- Health check configuration
- Status reporting

### Metrics (`cfg_svc_metrics`)

- Service metrics configuration
- Metrics endpoint management
- Performance monitoring setup

## Design Principles

1. **Context Awareness**
    - Automatic environment detection
    - Context-specific configuration
    - Seamless environment transitions

2. **Reliability**
    - Comprehensive error handling
    - Fallback configurations
    - Validation at all levels

3. **Security**
    - Secure credential management
    - Environment variable protection
    - Safe default configurations

4. **Flexibility**
    - Multiple environment support
    - Custom service configuration
    - Extensible design

## Development

### Prerequisites

- Rust 1.84.0 or higher
- Environment variables set according to context
- Network access for DNS resolution

### Testing

```bash
cargo test
```

### Debug Mode

Enable debug mode for detailed configuration information:

```rust
let cfg = CfgManager::with_debug(service_id, service_config);
```

## Error Handling

The component uses `InitError` for configuration-related errors:

- Environment variable missing
- DNS resolution failure
- Service configuration error
- Port binding failure

## Performance Considerations

1. **DNS Resolution**
    - Efficient resolver implementation
    - Caching where appropriate
    - Minimal network requests

2. **Configuration Loading**
    - Lazy loading when possible
    - Minimal resource usage
    - Efficient validation

3. **Service Discovery**
    - Fast endpoint resolution
    - Efficient dependency tracking
    - Optimized network queries
