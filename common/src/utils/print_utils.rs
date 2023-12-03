use crate::prelude::ServiceID;

pub fn print_start_header(service_id: &ServiceID,
                          service_port: u16,
                          metrics_uri: &String,
                          metrics_port: u16
)
{
    println!("==========================================");
    println!("Service {} on port {}", service_id, service_port);
    println!("Metrics on endpoint :{}/{}", metrics_port, metrics_uri);
    println!("==========================================");
    println!();
}

pub fn print_stop_header(service_id: &ServiceID) {
    println!();
    println!("==========================================");
    println!("{} service shutdown complete", service_id);
    println!("==========================================");
}