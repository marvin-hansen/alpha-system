use autometrics::autometrics;
use proto::binding::ims_data_service_server::ImsDataService;
use proto::binding::*;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct ImsDataServer {}

impl ImsDataServer {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
#[autometrics]
impl ImsDataService for ImsDataServer {
    async fn start_data(
        &self,
        request: Request<ProtoStartDataRequest>,
    ) -> Result<Response<ProtoStartDataResponse>, Status> {
        println!("[ImsDataBinance]: Processing request, {:?}", request);

        todo!()
    }

    async fn stop_data(
        &self,
        request: Request<ProtoStopDataRequest>,
    ) -> Result<Response<ProtoStopDataResponse>, Status> {
        println!("[ImsDataBinance]: Processing request, {:?}", request);
        todo!()
    }

    async fn start_all_data(
        &self,
        request: Request<ProtoStartAllDataRequest>,
    ) -> Result<Response<ProtoStartAllDataResponse>, Status> {
        println!("[ImsDataBinance]: Processing request, {:?}", request);
        todo!()
    }

    async fn stop_all_data(
        &self,
        request: Request<ProtoStopAllDataRequest>,
    ) -> Result<Response<ProtoStopAllDataResponse>, Status> {
        println!("[ImsDataBinance]: Processing request, {:?}", request);
        todo!()
    }

    async fn stop_exchange(
        &self,
        request: Request<ProtoStopExchangeRequest>,
    ) -> Result<Response<ProtoStopExchangeResponse>, Status> {
        println!("[ImsDataBinance]: Processing request, {:?}", request);
        todo!()
    }
}
