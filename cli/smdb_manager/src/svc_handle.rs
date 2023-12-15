use crate::svc_types::ServiceOP;
use common::prelude::ServiceID;
use smdb_provider::SMDBProvider;
use std::error::Error;

pub async fn handle_service_op(client: &SMDBProvider, op: ServiceOP) -> Result<(), Box<dyn Error>> {
    match op {
        ServiceOP::CheckIfServiceIDExists => check_if_svc_exists(client).await,
        ServiceOP::CheckIfServicesExists => check_if_svcs_exists(client).await,
        ServiceOP::CheckServiceIDOnline => check_if_svc_online(client).await,
        ServiceOP::CheckServicesOnline => check_if_svcs_online(client).await,
        ServiceOP::SetServiceOnline => set_svc_online(client).await,
        ServiceOP::SetServiceOffline => set_svc_offline(client).await,
    }

    Ok(())
}

async fn check_if_svc_exists(client: &SMDBProvider) {
    println!("Checking if service id exists");
    let id = ServiceID::SMDB;
    let exists = client
        .check_if_service_id_exists(id)
        .await
        .expect("Failed to check if service id exists");

    println!("Service id {:?} exists: {}", id, exists);

    println!("Checking if service id DOES NOT exists");
    let id = ServiceID::Default;
    let exists = client
        .check_if_service_id_exists(id)
        .await
        .expect("Failed to check if service id exists");

    println!("Service id {:?} exists: {}", id, exists);
}

async fn check_if_svcs_exists(client: &SMDBProvider) {
    println!("Checking if all services exist");
    let services = vec![ServiceID::SMDB, ServiceID::CMDB, ServiceID::DBGW];
    let exists = client
        .check_if_services_exists(services)
        .await
        .expect("Failed to check if services exists");

    println!("All Services exists: {}", &exists);
}

async fn set_svc_offline(client: &SMDBProvider) {
    println!("Checking if service is online");
    let id = ServiceID::SMDB;
    let online = client
        .check_if_service_id_online(id)
        .await
        .expect("Failed to check service id online");
    println!("Service id {:?} is online: {}", id, online);

    println!("Setting service offline");
    let id = ServiceID::SMDB;
    let _ = client
        .set_service_offline(id)
        .await
        .expect("Failed to set service online");

    println!("Checking if service is online");
    let id = ServiceID::SMDB;
    let online = client
        .check_if_service_id_online(id)
        .await
        .expect("Failed to check service id online");
    println!("Service id {:?} is online: {}", id, online);
}

async fn set_svc_online(client: &SMDBProvider) {
    println!("Checking if service is online");
    let id = ServiceID::SMDB;
    let online = client
        .check_if_service_id_online(id)
        .await
        .expect("Failed to check service id online");
    println!("Service id {:?} is online: {}", id, online);

    println!("Setting service online");
    let id = ServiceID::SMDB;
    let res = client
        .set_service_online(id)
        .await
        .expect("Failed to set service online");
    println!("Service online: {}", res);
}

async fn check_if_svcs_online(client: &SMDBProvider) {
    println!("Checking if all services are online");
    let services = vec![ServiceID::SMDB, ServiceID::CMDB, ServiceID::DBGW];
    let online = client
        .check_if_services_online(services)
        .await
        .expect("Failed to check services online");
    println!("All services are online: {}", &online);
}

async fn check_if_svc_online(client: &SMDBProvider) {
    println!("Checking if service id is online");
    let id = ServiceID::SMDB;
    let online = client
        .check_if_service_id_online(id)
        .await
        .expect("Failed to check service id online");
    println!("Service id {:?} is online: {}", id, online);
}
