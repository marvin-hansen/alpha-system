use std::error::Error;
use std::{fmt, process, time::Duration};
use tonic::transport::Channel;
use tonic::Request;
use tonic_health::pb::{
    health_check_response::ServingStatus, health_client::HealthClient, HealthCheckRequest,
};

const ERROR_CODE_CONNECTION_FAILURE: i32 = 2;
const ERROR_CODE_RPC_FAILURE: i32 = 3;

const CONNECTION_TIMEOUT: u64 = 250;
const RPC_TIMEOUT: u64 = 100;

#[derive(Debug, Clone)]
pub struct GrpcHealthError(pub String);

impl GrpcHealthError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for GrpcHealthError {}

impl fmt::Display for GrpcHealthError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GrpcHealthError: {}", self.0)
    }
}

/// Checks the health of a gRPC service at the specified address.
///
/// # Parameters
///
/// * `addr` - The address of the gRPC service to check.
///
/// # Panics
///
/// The function will terminate the process with an error code if it fails to connect,
/// the service is unhealthy, or the RPC call fails.
///
/// # Returns
///
/// Returns `Ok(())` if the service is healthy.
///
pub async fn check_grpc_health(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let channel_builder = Channel::from_shared(addr.clone())?
        .connect_timeout(Duration::from_millis(CONNECTION_TIMEOUT))
        .timeout(Duration::from_millis(RPC_TIMEOUT));

    let channel = match channel_builder.connect().await {
        Ok(ch) => ch,
        Err(err) => {
            eprintln!(
                "error: failed to connect service at {}: {:?}",
                addr.clone(),
                err
            );
            process::exit(ERROR_CODE_CONNECTION_FAILURE);
        }
    };

    let mut client = HealthClient::new(channel);

    let request = Request::new(HealthCheckRequest {
        service: "".to_string(),
    });

    match client.check(request).await {
        Ok(response) => {
            let status = response.into_inner().status();
            match status {
                ServingStatus::Serving => Ok(()),
                _ => {
                    eprintln!("service unhealthy (responded with {:?})", status);
                    Err(GrpcHealthError::new("service unhealthy".to_string()).into())
                }
            }
        }
        Err(status) => {
            match status.code() {
                tonic::Code::Unimplemented =>
                    eprintln!("error: this server does not implement the grpc health protocol (grpc.health.v1.Health): {}", status.message()),
                tonic::Code::DeadlineExceeded => println!("timeout: health rpc did not complete within {}", RPC_TIMEOUT),
                _ => eprintln!("error: health rpc failed: {}", status.message()),
            };
            process::exit(ERROR_CODE_RPC_FAILURE);
        }
    }
}
