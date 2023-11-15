use common::prelude::ServiceID;
use db_gateway_client::DBGatewayClient;
use specs::services::get_all_service_configs;

use crate::types::ServiceOP;

pub async fn handle_service_op(client: &DBGatewayClient, op: ServiceOP) -> anyhow::Result<()> {
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
            panic!("Not implemented yet");
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
            println!("{:?}", service.name());
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
