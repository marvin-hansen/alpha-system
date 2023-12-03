use crate::prelude::ServiceID;

pub fn print_start_header(service_id: &ServiceID, port: u16) {
    println!("==========================================");
    println!("Service {} on port {}", service_id, port);
    println!("==========================================");
    println!();
}

pub fn print_stop_header(service_id: &ServiceID) {
    println!();
    println!("==========================================");
    println!("{} service shutdown complete", service_id);
    println!("==========================================");
}