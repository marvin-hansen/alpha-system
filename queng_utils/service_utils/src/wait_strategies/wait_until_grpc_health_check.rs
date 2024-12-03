use crate::error::service_util_error::ServiceUtilError;
use crate::ServiceUtil;
use std::time::Duration;
use tokio::time::Instant;
use tonic::transport::Channel;
use tonic_health::pb::health_client::HealthClient;
use tonic_health::pb::HealthCheckRequest;

impl ServiceUtil {
    /// Waits until a gRPC service at the given `health_url` is online by performing
    /// a health check. The health check is performed every 100ms until the service
    /// responds or the given `timeout` is reached. If the timeout is reached without
    /// a response, a `ServiceUtilError` is returned.
    ///
    /// The health check is done by calling the `Check` method on the gRPC health
    /// service. The `service` parameter of the `HealthCheckRequest` is an empty string.
    ///
    pub(crate) async fn wait_until_grpc_health_check(
        &self,
        health_url: &str,
        timeout: &Duration,
    ) -> Result<(), ServiceUtilError> {
        let start_time = Instant::now();

        loop {
            tokio::time::sleep(Duration::from_millis(50)).await;

            if start_time.elapsed().as_secs() > timeout.as_secs() {
                return Err(ServiceUtilError::ServiceHealthcheckFailed(format!(
                    "[wait_until_grpc_health_check]: !!Timeout!! Waited {} seconds for service health check",
                    timeout.as_secs(),
                )));
            }

            match Channel::from_shared(health_url.to_string())
                .expect("valid uri")
                .connect()
                .await
            {
                Ok(channel) => {
                    let mut client = HealthClient::new(channel);
                    let request = tonic::Request::new(HealthCheckRequest {
                        service: "".to_string(), // Default setting. Works for all services.
                    });

                    match client.check(request).await {
                        Ok(_) => {
                            self.dbg_print("gRPC Service online");
                            break Ok(());
                        }
                        Err(e) => {
                            self.dbg_print(&format!(
                                "[wait_until_grpc_health_check]: Health check failed: {}",
                                e
                            ));
                            continue;
                        }
                    }
                }
                Err(e) => {
                    self.dbg_print(&format!(
                        "[wait_until_grpc_health_check]: Connection failed: {}",
                        e
                    ));
                    continue;
                }
            }
        }
    }
}
