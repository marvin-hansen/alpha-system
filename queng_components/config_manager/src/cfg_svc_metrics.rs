use crate::CfgManager;
use common_errors::InitError;

impl CfgManager {
    /// Returns the metric socket address and uri to run the service in any
    pub fn get_metrics_socket_addr_uri(&self) -> Result<(String, String), InitError> {
        let (metrics_host, metrics_uri, metrics_port) = self
            .get_svc_metric_host_uri_port()
            .expect("Failed to get metric host, uri, and port");

        // Merge the host and port into a socket address i.e. 0.0.0.0:8080
        let socket_addr = format!("{}:{}", metrics_host, metrics_port);

        Ok((socket_addr, metrics_uri))
    }

    pub(super) fn get_svc_metric_host_uri_port(&self) -> Result<(String, String, u32), InitError> {
        let svc = self.svc_env_config.to_owned();
        let metric_host = svc.metrics_host().to_string();
        let metrics_uri = svc.metrics_uri().to_string();
        let port = *svc.metrics_port() as u16;

        let metrics_port = self
            .get_port(port, &self.svc)
            .expect("[EnvManager]: Failed to get port from config");

        Ok((metric_host, metrics_uri, metrics_port as u32))
    }
}
