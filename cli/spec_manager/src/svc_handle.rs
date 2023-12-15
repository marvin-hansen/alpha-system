use common::prelude::ServiceID;
use dbgw_client::DBGatewayClient;
use specs::prelude::smdb_service_config;
use specs::services::get_all_service_configs;
use std::error::Error;

use crate::types::ServiceOP;

pub async fn handle_service_op(
    client: &DBGatewayClient,
    op: ServiceOP,
) -> Result<(), Box<dyn Error>> {
    let mut client = client.clone();

    match op {
        ServiceOP::CreateAllService => {
            println!("Creating all services");
            let services = get_all_service_configs();
            for service in services {
                println!("{:?}", service.name());
                client
                    .create_service(service)
                    .await
                    .expect("Failed to create service");
            }
        }
        ServiceOP::CreateService => {
            let service = smdb_service_config();
            let res = client
                .create_service(service)
                .await
                .expect("Failed to create service");

            println!("Service created: {}", res);
        }

        ServiceOP::CheckIfServiceIDExists => {
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

        ServiceOP::CheckIfServicesExists => {
            println!("Checking if all services exist");
            let services = vec![ServiceID::SMDB, ServiceID::CMDB, ServiceID::DBGW];
            let exists = client
                .check_if_services_exists(services)
                .await
                .expect("Failed to check if services exists");

            println!("All Services exists: {}", &exists);
        }

        ServiceOP::CheckServiceIDOnline => {
            println!("Checking if service id is online");
            let id = ServiceID::SMDB;
            let online = client
                .check_if_service_id_online(id)
                .await
                .expect("Failed to check service id online");
            println!("Service id {:?} is online: {}", id, online);
        }

        ServiceOP::CheckServicesOnline => {
            println!("Checking if all services are online");
            let services = vec![ServiceID::SMDB, ServiceID::CMDB, ServiceID::DBGW];
            let online = client
                .check_if_services_online(services)
                .await
                .expect("Failed to check services online");
            println!("All services are online: {}", &online);
        }

        ServiceOP::ReadAllServices => {
            println!("Reading all services");
            let services = client
                .read_all_services()
                .await
                .expect("Failed to read all services");

            for service in services {
                println!("{:?}", service.name());
            }
        }
        ServiceOP::ReadServiceById => {
            println!("Reading service by id");
            let id = ServiceID::SMDB;
            let service = client
                .read_service_by_id(id)
                .await
                .expect("Failed to read service by id");
            println!("{:?}", service);
        }
        ServiceOP::SetServiceOnline => {
            println!("Setting service online");
            let id = ServiceID::SMDB;
            let res = client
                .set_service_online(id)
                .await
                .expect("Failed to set service online");
            println!("Service online: {}", res);

            println!("read & print service");
            let id = ServiceID::SMDB;
            let service = client
                .read_service_by_id(id)
                .await
                .expect("Failed to read service by id");
            println!("{:?}", service);
        }

        ServiceOP::SetServiceOffline => {
            println!("Setting service offline");
            let id = ServiceID::SMDB;
            let res = client
                .set_service_offline(id)
                .await
                .expect("Failed to set service online");

            println!("Service offline: {}", res);

            let id = ServiceID::SMDB;
            let service = client
                .read_service_by_id(id)
                .await
                .expect("Failed to read service by id");
            println!("{:?}", service);
        }

        ServiceOP::UpdateService => {
            panic!("Not implemented yet");
        }
        ServiceOP::DeleteService => {
            println!("Deleting service");
            let id = ServiceID::SMDB;
            let deleted = client
                .delete_service(id)
                .await
                .expect("Failed to delete service");
            println!("Services deleted: {:?}", deleted);
        }
        ServiceOP::DeleteAllServices => {
            println!("Deleting all services");
            let services = client
                .clone()
                .read_all_services()
                .await
                .expect("Failed to read all services");
            for service in services {
                println!("Deleting {:?}", service.name());
                client
                    .delete_service(*service.svc_id())
                    .await
                    .expect("Failed to delete service");
            }
        }
    }

    Ok(())
}
