use crate::prelude::ServiceID;

pub fn print_start_header(service_id: &ServiceID, port: u16) {
    println!();
    println!("==========================================");
    println!("Service {} on port {}", service_id, port);
    println!("==========================================");
    println!();
}
