use common::prelude::{ServiceConfig, ServiceID};
use proto_binding::dbgw::{MultiServicesRequest, SingleServiceRequest};

use crate::{DBGatewayClient, DBGatewayError};

impl DBGatewayClient {
    pub async fn create_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let proto_service_config = data
            .to_proto()
            .expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_service_config);

        let res = self.client.borrow_mut().create_service(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().service_created),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn check_if_service_id_exists(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let res = self
            .client
            .borrow_mut()
            .check_service_id_exists(request)
            .await;

        match res {
            Ok(res) => Ok(res.into_inner().service_exists),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn check_if_services_exists(
        &self,
        services: Vec<ServiceID>,
    ) -> Result<bool, DBGatewayError> {
        let services_id = services
            .iter()
            .map(|s| s.to_owned() as i32)
            .collect::<Vec<i32>>();

        let request = tonic::Request::new(MultiServicesRequest { services_id });

        let res = self
            .client
            .borrow_mut()
            .check_services_exists(request)
            .await;

        match res {
            Ok(res) => Ok(res.into_inner().services_exist),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn check_if_service_id_online(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let res = self
            .client
            .borrow_mut()
            .check_service_id_online(request)
            .await;

        match res {
            Ok(res) => Ok(res.into_inner().service_online),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }
    pub async fn check_if_services_online(
        &self,
        services: Vec<ServiceID>,
    ) -> Result<bool, DBGatewayError> {
        let services_id = services
            .iter()
            .map(|s| s.to_owned() as i32)
            .collect::<Vec<i32>>();

        let request = tonic::Request::new(MultiServicesRequest { services_id });

        let res = self
            .client
            .borrow_mut()
            .check_services_online(request)
            .await;

        match res {
            Ok(res) => Ok(res.into_inner().services_online),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, DBGatewayError> {
        let services_id = Vec::new();
        let request = tonic::Request::new(MultiServicesRequest { services_id });

        let res = self.client.borrow_mut().read_all_services(request).await;

        match res {
            Ok(res) => {
                let vec = res
                    .into_inner()
                    .service_configs
                    .iter()
                    .map(|s| {
                        ServiceConfig::from_proto(s.to_owned())
                            .expect("Failed to convert ProtoServiceConfig to Rust ServiceConfig")
                    })
                    .collect::<Vec<ServiceConfig>>();

                Ok(vec)
            }
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn read_service_by_id(
        &self,
        id: ServiceID,
    ) -> Result<Option<ServiceConfig>, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let res = self.client.borrow_mut().read_service(request).await;

        match res {
            Ok(res) => match res.into_inner().service_config {
                Some(p) => {
                    let service_config = ServiceConfig::from_proto(p.to_owned())
                        .expect("Failed to convert ProtoServiceConfig to Rust ServiceConfig");

                    Ok(Some(service_config))
                }
                None => Ok(None),
            },
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn set_service_online(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let res = self.client.borrow_mut().set_service_online(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().service_online),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn set_service_offline(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let res = self.client.borrow_mut().set_service_offline(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().service_offline),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn update_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let proto_service_config = data
            .to_proto()
            .expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_service_config);

        let res = self.client.borrow_mut().update_service(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().service_updated),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn delete_service(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let res = self.client.borrow_mut().delete_service(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().service_deleted),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }
}
