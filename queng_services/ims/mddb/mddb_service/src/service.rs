use proto_mddb::proto::db_gateway_mddb_service_client::DbGatewayMddbServiceClient;
use proto_mddb::proto::mddb_service_server::MddbService;
use proto_mddb::proto::*;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct MDDBServer {
    dbgw: DbGatewayMddbServiceClient<Channel>,
}

impl MDDBServer {
    pub fn new(dbgw: DbGatewayMddbServiceClient<Channel>) -> Self {
        Self { dbgw }
    }
}

#[tonic::async_trait]
impl MddbService for MDDBServer {
    async fn count_assets(
        &self,
        request: Request<CountAssetsRequest>,
    ) -> Result<Response<CountAssetsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.count_assets(request).await {
            Ok(res) => Ok(Response::new(CountAssetsResponse {
                count: res.into_inner().count,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_asset_id_exists(
        &self,
        request: Request<CheckIfAssetIdExistsRequest>,
    ) -> Result<Response<CheckIfAssetIdExistsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_if_asset_id_exists(request).await {
            Ok(res) => Ok(Response::new(CheckIfAssetIdExistsResponse {
                asset_id: res.get_ref().asset_id.clone(),
                exists: res.get_ref().exists,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_asset(
        &self,
        request: Request<GetAssetRequest>,
    ) -> Result<Response<GetAssetResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_asset(request).await {
            Ok(res) => Ok(Response::new(GetAssetResponse {
                asset_id: res.get_ref().asset_id.clone(),
                asset: res.into_inner().asset,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_assets(
        &self,
        request: Request<GetAllAssetsRequest>,
    ) -> Result<Response<GetAllAssetsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_assets(request).await {
            Ok(res) => Ok(Response::new(GetAllAssetsResponse {
                assets: res.into_inner().assets,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn count_exchanges(
        &self,
        request: Request<CountExchangesRequest>,
    ) -> Result<Response<CountExchangesResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.count_exchanges(request).await {
            Ok(res) => Ok(Response::new(CountExchangesResponse {
                count: res.into_inner().count,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_exchange_id_exists(
        &self,
        request: Request<CheckIfExchangeIdExistsRequest>,
    ) -> Result<Response<CheckIfExchangeIdExistsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_if_exchange_id_exists(request).await {
            Ok(res) => Ok(Response::new(CheckIfExchangeIdExistsResponse {
                exchange_code: res.get_ref().exchange_code.clone(),
                exists: res.get_ref().exists,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_exchange(
        &self,
        request: Request<GetExchangeRequest>,
    ) -> Result<Response<GetExchangeResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_exchange(request).await {
            Ok(res) => Ok(Response::new(GetExchangeResponse {
                exchange: res.into_inner().exchange,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_exchanges(
        &self,
        request: Request<GetAllExchangesRequest>,
    ) -> Result<Response<GetAllExchangesResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_exchanges(request).await {
            Ok(res) => Ok(Response::new(GetAllExchangesResponse {
                exchanges: res.into_inner().exchanges,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn lookup_exchange_name(
        &self,
        request: Request<LookupExchangeNameRequest>,
    ) -> Result<Response<LookupExchangeNameResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.lookup_exchange_name(request).await {
            Ok(res) => Ok(Response::new(res.into_inner())),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn count_instruments(
        &self,
        request: Request<CountInstrumentsRequest>,
    ) -> Result<Response<CountInstrumentsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.count_instruments(request).await {
            Ok(res) => Ok(Response::new(CountInstrumentsResponse {
                count: res.into_inner().count,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_instrument_id_exists(
        &self,
        request: Request<CheckIfInstrumentIdExistsRequest>,
    ) -> Result<Response<CheckIfInstrumentIdExistsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_if_instrument_id_exists(request).await {
            Ok(res) => Ok(Response::new(CheckIfInstrumentIdExistsResponse {
                instrument_id: res.get_ref().instrument_id.clone(),
                exists: res.get_ref().exists,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_instrument(
        &self,
        request: Request<GetInstrumentByIdRequest>,
    ) -> Result<Response<GetInstrumentByIdResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_instrument(request).await {
            Ok(res) => Ok(Response::new(GetInstrumentByIdResponse {
                instrument: res.into_inner().instrument,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_instrument_by_figi(
        &self,
        request: Request<GetInstrumentByFigiRequest>,
    ) -> Result<Response<GetInstrumentByIdResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_instrument_by_figi(request).await {
            Ok(res) => Ok(Response::new(GetInstrumentByIdResponse {
                instrument: res.into_inner().instrument,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments(
        &self,
        request: Request<GetAllInstrumentsRequest>,
    ) -> Result<Response<GetAllInstrumentsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_instruments(request).await {
            Ok(res) => Ok(Response::new(GetAllInstrumentsResponse {
                instruments: res.into_inner().instruments,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_base_asset(
        &self,
        request: Request<GetAllInstrumentsForBaseAssetRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseAssetResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_instruments_for_base_asset(request).await {
            Ok(res) => Ok(Response::new(GetAllInstrumentsForBaseAssetResponse {
                instruments: res.into_inner().instruments,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_quote_asset(
        &self,
        request: Request<GetAllInstrumentsForQuoteAssetRequest>,
    ) -> Result<Response<GetAllInstrumentsForQuoteAssetResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_instruments_for_quote_asset(request).await {
            Ok(res) => Ok(Response::new(GetAllInstrumentsForQuoteAssetResponse {
                instruments: res.into_inner().instruments,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_exchange(
        &self,
        request: Request<GetAllInstrumentsForExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForExchangeResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_instruments_for_exchange(request).await {
            Ok(res) => Ok(Response::new(GetAllInstrumentsForExchangeResponse {
                instruments: res.into_inner().instruments,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_base_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForBaseAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseAssetAndExchangeResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client
            .get_all_instruments_for_base_asset_and_exchange(request)
            .await
        {
            Ok(res) => Ok(Response::new(
                GetAllInstrumentsForBaseAssetAndExchangeResponse {
                    instruments: res.into_inner().instruments,
                },
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_quote_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForQuoteAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForQuoteAssetAndExchangeResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client
            .get_all_instruments_for_quote_asset_and_exchange(request)
            .await
        {
            Ok(res) => Ok(Response::new(
                GetAllInstrumentsForQuoteAssetAndExchangeResponse {
                    instruments: res.into_inner().instruments,
                },
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_base_quote_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client
            .get_all_instruments_for_base_quote_asset_and_exchange(request)
            .await
        {
            Ok(res) => Ok(Response::new(
                GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse {
                    instruments: res.into_inner().instruments,
                },
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn lookup_instrument_exchange_pair_code(
        &self,
        request: Request<LookupInstrumentExchangePairCodeRequest>,
    ) -> Result<Response<LookupInstrumentExchangePairCodeResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.lookup_instrument_exchange_pair_code(request).await {
            Ok(res) => Ok(Response::new(res.into_inner())),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn lookup_instrument_figi(
        &self,
        request: Request<LookupInstrumentFigiRequest>,
    ) -> Result<Response<LookupInstrumentFigiResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.lookup_instrument_figi(request).await {
            Ok(res) => Ok(Response::new(res.into_inner())),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
