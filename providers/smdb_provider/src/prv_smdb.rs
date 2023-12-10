use common::prelude::ServiceID;
use proto::binding::{MultiServicesRequest, SingleServiceRequest};

use crate::{SMDBError, SMDBProvider};

impl SMDBProvider {
    pub async fn check_if_service_id_exists(&self, id: ServiceID) -> Result<bool, SMDBError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.check_service_id_exists(request).await {
            Ok(res) => Ok(res.into_inner().service_exists),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    pub async fn check_if_services_exists(
        &self,
        services: Vec<ServiceID>,
    ) -> Result<bool, SMDBError> {
        let services_id = services.iter().map(|s| s.to_owned() as i32).collect();

        let request = tonic::Request::new(MultiServicesRequest { services_id });

        let mut client = self.client.clone();

        match client.check_services_exists(request).await {
            Ok(res) => Ok(res.into_inner().services_exist),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    pub async fn check_if_service_id_online(&self, id: ServiceID) -> Result<bool, SMDBError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.check_service_id_online(request).await {
            Ok(res) => Ok(res.into_inner().service_online),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    pub async fn check_if_services_online(
        &self,
        services: Vec<ServiceID>,
    ) -> Result<bool, SMDBError> {
        let services_id = services.iter().map(|s| s.to_owned() as i32).collect();

        let request = tonic::Request::new(MultiServicesRequest { services_id });

        let mut client = self.client.clone();

        match client.check_services_online(request).await {
            Ok(res) => Ok(res.into_inner().services_online),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    pub async fn set_service_online(&self, id: ServiceID) -> Result<bool, SMDBError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.set_service_online(request).await {
            Ok(res) => Ok(res.into_inner().service_online),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    pub async fn set_service_offline(&self, id: ServiceID) -> Result<bool, SMDBError> {
        let request = tonic::Request::new(SingleServiceRequest {
            service_id: id as i32,
        });

        let mut client = self.client.clone();

        match client.set_service_offline(request).await {
            Ok(res) => Ok(res.into_inner().service_offline),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }
}
