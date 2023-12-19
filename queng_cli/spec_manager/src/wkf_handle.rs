use common::prelude::ServiceID;
use dbgw_client::DBGatewayClient;
use service_specs::services::get_all_service_configs;
use std::error::Error;

use crate::types::WorkflowOP;

pub async fn handle_workflow_op(
    client: &DBGatewayClient,
    op: WorkflowOP,
) -> Result<(), Box<dyn Error>> {
    match op {
        WorkflowOP::CreateRead => {
            create_read(client).await?;
        }

        WorkflowOP::SetCheckOnline => {
            set_check_online(client).await?;
        }
    }

    Ok(())
}

async fn create_read(client: &DBGatewayClient) -> Result<(), Box<dyn Error>> {
    let mut client = client.clone();

    println!("Creating all services");
    let services = get_all_service_configs();
    for service in services {
        println!("{:?}", service.name());
        client
            .create_service(service)
            .await
            .expect("Failed to create service");
    }

    println!("Checking if all services exist");
    let services = vec![ServiceID::SMDB, ServiceID::CMDB, ServiceID::DBGW];
    let exists = client
        .check_if_services_exists(services)
        .await
        .expect("Failed to check if services exists");

    println!("All Services exists: {}", &exists);
    Ok(())
}

async fn set_check_online(client: &DBGatewayClient) -> Result<(), Box<dyn Error>> {
    println!("Checking if service id exists");
    let id = ServiceID::SMDB;
    let exists = client
        .check_if_service_id_exists(id)
        .await
        .expect("Failed to check if service id exists");

    if !exists {
        create_read(client).await?;
    }

    let exists = client
        .check_if_service_id_exists(id)
        .await
        .expect("Failed to check if service id exists");

    println!("Service id {:?} exists: {}", id, exists);
    println!();

    println!("Checking if service id is online");
    let id = ServiceID::SMDB;
    let online = client
        .check_if_service_id_online(id)
        .await
        .expect("Failed to check service id online");
    println!("Service id {:?} is online: {}", id, online);
    println!();

    println!("Setting service online");
    let id = ServiceID::SMDB;
    let res = client
        .set_service_online(id)
        .await
        .expect("Failed to set service online");
    println!("Service online: {}", res);
    println!();

    println!("Checking if service id is online");
    let id = ServiceID::SMDB;
    let online = client
        .check_if_service_id_online(id)
        .await
        .expect("Failed to check service id online");
    println!("Service id {:?} is online: {}", id, online);
    println!();

    println!("read & print service");
    let id = ServiceID::SMDB;
    let service = client
        .read_service_by_id(id)
        .await
        .expect("Failed to read service by id");
    println!("{:?}", service);
    println!();

    Ok(())
}
