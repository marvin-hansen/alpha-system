use common_config::prelude::{Endpoint, ServiceConfig, ServiceID};
use proto_smdb::proto::{
    CountServiceRequest, MultiServicesRequest, ServiceDependenciesRequest, ServiceEndpointsRequest,
    ServicesOfflineRequest, ServicesOnlineRequest, SingleServiceRequest,
};
use proto_smdb_utils::endpoint_proto_utils::endpoint_from_proto;
use proto_smdb_utils::service_config_proto_utils::{
    service_config_from_proto, service_config_to_proto,
};

use crate::DBGatewayClient;
use common_errors::prelude::DBGatewayError;

impl DBGatewayClient {
    pub async fn create_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let proto_service_config =
            service_config_to_proto(data).expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_service_config);

        let mut client = self.client.clone();

        match client.create_service(request).await {
            Ok(res) => Ok(res.into_inner().service_created),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn count_all_services(self) -> Result<u64, DBGatewayError> {
        let request = tonic::Request::new(CountServiceRequest {});

        let mut client = self.client.clone();

        match client.count_all_services(request).await {
            Ok(res) => Ok(res.into_inner().service_count),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn check_if_service_id_exists(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.check_service_id_exists(request).await {
            Ok(res) => Ok(res.into_inner().service_exists),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn check_if_services_exists(
        &mut self,
        services: Vec<ServiceID>,
    ) -> Result<bool, DBGatewayError> {
        let services_id = services
            .iter()
            .map(|s| s.to_owned() as i32)
            .collect::<Vec<i32>>();

        let request = tonic::Request::new(MultiServicesRequest { services_id });

        match self.client.check_services_exists(request).await {
            Ok(res) => Ok(res.into_inner().services_exist),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn check_if_service_id_online(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.check_service_id_online(request).await {
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

        let mut client = self.client.clone();

        match client.check_services_online(request).await {
            Ok(res) => Ok(res.into_inner().services_online),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn get_all_service_dependencies(
        self,
        id: ServiceID,
    ) -> Result<Vec<ServiceID>, DBGatewayError> {
        let request = tonic::Request::new(ServiceDependenciesRequest {
            service_id: id.as_i32(),
        });

        let mut client = self.client.clone();

        match client.get_all_service_dependencies(request).await {
            Ok(res) => {
                // Convert i32 back into ServiceID
                let vec = res
                    .into_inner()
                    .dependencies
                    .iter()
                    .map(|s| ServiceID::from(*s))
                    .collect::<Vec<ServiceID>>();

                Ok(vec)
            }
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn get_all_service_endpoints(
        self,
        id: ServiceID,
    ) -> Result<Vec<Endpoint>, DBGatewayError> {
        let request = tonic::Request::new(ServiceEndpointsRequest {
            service_id: id.as_i32(),
        });

        let mut client = self.client.clone();

        match client.get_all_service_endpoints(request).await {
            Ok(res) => {
                // Convert proto endpoints back to Rust endpoints
                let vec = endpoint_from_proto(res.into_inner().endpoints)
                    .expect("Failed to convert ProtoEndpoints to Rust EndpointCollection");

                Ok(vec)
            }
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn get_all_online_services(self) -> Result<Vec<ServiceConfig>, DBGatewayError> {
        let request = tonic::Request::new(ServicesOnlineRequest {});

        let mut client = self.client.clone();

        match client.get_all_online_services(request).await {
            Ok(res) => {
                let vec = res
                    .into_inner()
                    .service_configs
                    .iter()
                    .map(|s| {
                        service_config_from_proto(s.to_owned())
                            .expect("Failed to convert ProtoServiceConfig to Rust ServiceConfig")
                    })
                    .collect::<Vec<ServiceConfig>>();

                Ok(vec)
            }
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn get_all_offline_services(self) -> Result<Vec<ServiceConfig>, DBGatewayError> {
        let request = tonic::Request::new(ServicesOfflineRequest {});

        let mut client = self.client.clone();
        match client.get_all_offline_services(request).await {
            Ok(res) => {
                let vec = res
                    .into_inner()
                    .service_configs
                    .iter()
                    .map(|s| {
                        service_config_from_proto(s.to_owned())
                            .expect("Failed to convert ProtoServiceConfig to Rust ServiceConfig")
                    })
                    .collect::<Vec<ServiceConfig>>();

                Ok(vec)
            }
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn read_all_services(self) -> Result<Vec<ServiceConfig>, DBGatewayError> {
        let services_id = Vec::new();
        let request = tonic::Request::new(MultiServicesRequest { services_id });

        let mut client = self.client.clone();

        match client.read_all_services(request).await {
            Ok(res) => {
                let vec = res
                    .into_inner()
                    .service_configs
                    .iter()
                    .map(|s| {
                        service_config_from_proto(s.to_owned())
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
        svc_id: ServiceID,
    ) -> Result<Option<ServiceConfig>, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: svc_id as i32,
        });

        let mut client = self.client.clone();

        match client.read_service(request).await {
            Ok(res) => match res.into_inner().service_config {
                Some(p) => {
                    let service_config = service_config_from_proto(p.to_owned())
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

        let mut client = self.client.clone();

        match client.set_service_online(request).await {
            Ok(res) => Ok(res.into_inner().service_online),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn set_service_offline(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.set_service_offline(request).await {
            Ok(res) => Ok(res.into_inner().service_offline),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn update_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let proto_service_config =
            service_config_to_proto(data).expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_service_config);

        let mut client = self.client.clone();

        match client.update_service(request).await {
            Ok(res) => Ok(res.into_inner().service_updated),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn delete_service(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.delete_service(request).await {
            Ok(res) => Ok(res.into_inner().service_deleted),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }
}
