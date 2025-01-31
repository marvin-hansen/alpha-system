/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::DBG;
use pg_mddb_manager::PostgresMDDBManager;
use proto_mddb::proto::db_gateway_mddb_service_server::DbGatewayMddbService;
use proto_mddb::proto::{
    CheckIfAssetIdExistsRequest, CheckIfAssetIdExistsResponse, CheckIfExchangeIdExistsRequest,
    CheckIfExchangeIdExistsResponse, CheckIfInstrumentIdExistsRequest,
    CheckIfInstrumentIdExistsResponse, CountAssetsRequest, CountAssetsResponse,
    CountExchangesRequest, CountExchangesResponse, CountInstrumentsRequest,
    CountInstrumentsResponse, GetAllAssetsRequest, GetAllAssetsResponse, GetAllExchangesRequest,
    GetAllExchangesResponse, GetAllInstrumentsForBaseAssetAndExchangeRequest,
    GetAllInstrumentsForBaseAssetAndExchangeResponse, GetAllInstrumentsForBaseAssetRequest,
    GetAllInstrumentsForBaseAssetResponse, GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest,
    GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse, GetAllInstrumentsForExchangeRequest,
    GetAllInstrumentsForExchangeResponse, GetAllInstrumentsForQuoteAssetAndExchangeRequest,
    GetAllInstrumentsForQuoteAssetAndExchangeResponse, GetAllInstrumentsForQuoteAssetRequest,
    GetAllInstrumentsForQuoteAssetResponse, GetAllInstrumentsRequest, GetAllInstrumentsResponse,
    GetAssetRequest, GetAssetResponse, GetExchangeRequest, GetExchangeResponse,
    GetInstrumentByFigiRequest, GetInstrumentByFigiResponse, GetInstrumentByIdRequest,
    GetInstrumentByIdResponse, GetInstrumentByPairFigiRequest, GetInstrumentByPairFigiResponse,
    LookupExchangeNameRequest, LookupExchangeNameResponse,
    LookupInstrumentIdByExchangePairCodeRequest, LookupInstrumentIdByExchangePairCodeResponse,
    LookupInstrumentIdByFigiRequest, LookupInstrumentIdByFigiResponse,
    LookupInstrumentIdByPairFigiRequest, LookupInstrumentIdByPairFigiResponse,
};
use proto_mddb_utils::{
    get_all_assets_response, get_all_exchanges_response,
    get_all_instruments_for_base_asset_and_exchange_response,
    get_all_instruments_for_base_asset_response,
    get_all_instruments_for_base_quote_asset_and_exchange_response,
    get_all_instruments_for_exchange_response,
    get_all_instruments_for_quote_asset_and_exchange_response,
    get_all_instruments_for_quote_asset_response, get_all_instruments_response,
    get_assets_response, get_check_if_asset_exists_response, get_check_if_exchange_exists_response,
    get_check_if_instrument_exists_response, get_count_assets_response,
    get_count_exchanges_response, get_count_instruments_response, get_exchange_response,
    get_instrument_by_figi_response, get_instrument_by_id_response,
    get_instrument_by_pair_figi_response, get_lookup_exchange_name_response,
    get_lookup_instrument_by_figi_response, get_lookup_instrument_by_pair_figi_response,
    get_lookup_instrument_id_by_exchange_pair_code_response,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

pub type SafePgMDDBManager = Arc<RwLock<PostgresMDDBManager>>;

#[derive(Clone)]
pub struct MDDBServer {
    dbg: bool,
    dbm: SafePgMDDBManager,
}

impl MDDBServer {
    pub const fn new(dbm: SafePgMDDBManager) -> Self {
        Self { dbg: DBG, dbm }
    }

    fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[DBGW/service_mddb]: {msg}");
        }
    }
}

#[tonic::async_trait]
impl DbGatewayMddbService for MDDBServer {
    async fn count_assets(
        &self,
        _request: Request<CountAssetsRequest>,
    ) -> Result<Response<CountAssetsResponse>, Status> {
        self.dbg_print("count_assets");

        let dbm = self.dbm.read().await;
        let res = dbm.count_assets().await;

        match res {
            Ok(count) => Ok(Response::new(get_count_assets_response(count))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_asset_id_exists(
        &self,
        request: Request<CheckIfAssetIdExistsRequest>,
    ) -> Result<Response<CheckIfAssetIdExistsResponse>, Status> {
        self.dbg_print("check_if_asset_id_exists");

        let asset_id = request.into_inner().asset_id;
        let dbm = self.dbm.read().await;
        let res = dbm.check_if_asset_id_exists(asset_id.clone()).await;

        match res {
            Ok(exists) => Ok(Response::new(get_check_if_asset_exists_response(exists))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_asset(
        &self,
        request: Request<GetAssetRequest>,
    ) -> Result<Response<GetAssetResponse>, Status> {
        self.dbg_print("get_asset");

        let asset_id = request.into_inner().asset_id;
        let dbm = self.dbm.read().await;
        let res = dbm.read_asset(asset_id.clone()).await;

        match res {
            Ok(asset) => Ok(Response::new(get_assets_response(asset))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_assets(
        &self,
        _request: Request<GetAllAssetsRequest>,
    ) -> Result<Response<GetAllAssetsResponse>, Status> {
        self.dbg_print("get_all_assets");

        let dbm = self.dbm.read().await;
        let res = dbm.read_all_assets().await;

        match res {
            Ok(assets) => Ok(Response::new(get_all_assets_response(&assets))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn count_exchanges(
        &self,
        _request: Request<CountExchangesRequest>,
    ) -> Result<Response<CountExchangesResponse>, Status> {
        self.dbg_print("count_exchanges");

        let dbm = self.dbm.read().await;
        let res = dbm.count_exchanges().await;

        match res {
            Ok(count) => Ok(Response::new(get_count_exchanges_response(count))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_exchange_id_exists(
        &self,
        request: Request<CheckIfExchangeIdExistsRequest>,
    ) -> Result<Response<CheckIfExchangeIdExistsResponse>, Status> {
        self.dbg_print("check_if_exchange_id_exists");

        let exchange_code = request.into_inner().exchange_code;
        let dbm = self.dbm.read().await;
        let res = dbm.check_if_exchange_id_exists(exchange_code.clone()).await;

        match res {
            Ok(exists) => Ok(Response::new(get_check_if_exchange_exists_response(exists))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_exchange(
        &self,
        request: Request<GetExchangeRequest>,
    ) -> Result<Response<GetExchangeResponse>, Status> {
        self.dbg_print("get_exchange");

        let exchange_code = request.into_inner().exchange_code;
        let dbm = self.dbm.read().await;
        let res = dbm.read_exchange(exchange_code.clone()).await;

        match res {
            Ok(exchange) => Ok(Response::new(get_exchange_response(exchange))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_exchanges(
        &self,
        _request: Request<GetAllExchangesRequest>,
    ) -> Result<Response<GetAllExchangesResponse>, Status> {
        self.dbg_print("get_all_exchanges");

        let dbm = self.dbm.read().await;
        let res = dbm.read_all_exchanges().await;

        match res {
            Ok(exchanges) => Ok(Response::new(get_all_exchanges_response(exchanges))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn lookup_exchange_name(
        &self,
        request: Request<LookupExchangeNameRequest>,
    ) -> Result<Response<LookupExchangeNameResponse>, Status> {
        self.dbg_print("lookup_exchange_name");

        let exchange_code = request.into_inner().exchange_code;
        let dbm = self.dbm.read().await;
        let res = dbm.read_exchange(exchange_code.clone()).await;

        match res {
            Ok(exchange) => Ok(Response::new(get_lookup_exchange_name_response(exchange))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn count_instruments(
        &self,
        _request: Request<CountInstrumentsRequest>,
    ) -> Result<Response<CountInstrumentsResponse>, Status> {
        self.dbg_print("count_instruments");

        let dbm = self.dbm.read().await;
        let res = dbm.count_instruments().await;

        match res {
            Ok(count) => Ok(Response::new(get_count_instruments_response(count))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_instrument_id_exists(
        &self,
        request: Request<CheckIfInstrumentIdExistsRequest>,
    ) -> Result<Response<CheckIfInstrumentIdExistsResponse>, Status> {
        self.dbg_print("check_if_instrument_id_exists");

        let instrument_id = request.into_inner().instrument_id;
        let dbm = self.dbm.read().await;
        let res = dbm.check_if_instrument_id_exists(&instrument_id).await;

        match res {
            Ok(exists) => Ok(Response::new(get_check_if_instrument_exists_response(
                &instrument_id,
                exists,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_instrument(
        &self,
        request: Request<GetInstrumentByIdRequest>,
    ) -> Result<Response<GetInstrumentByIdResponse>, Status> {
        self.dbg_print("get_instrument");

        let instrument_id = request.into_inner().instrument_id;
        let dbm = self.dbm.read().await;
        let res = dbm.read_instrument(&instrument_id).await;

        match res {
            Ok(instrument) => Ok(Response::new(get_instrument_by_id_response(instrument))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_instrument_by_figi(
        &self,
        request: Request<GetInstrumentByFigiRequest>,
    ) -> Result<Response<GetInstrumentByFigiResponse>, Status> {
        self.dbg_print("get_instrument_by_figi");
        let figi = request.into_inner().instrument_figi;
        let dbm = self.dbm.read().await;

        match dbm.read_instrument_by_figi(&figi).await {
            Ok(instrument) => Ok(Response::new(get_instrument_by_figi_response(instrument))),

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_instrument_by_pair_figi(
        &self,
        request: Request<GetInstrumentByPairFigiRequest>,
    ) -> Result<Response<GetInstrumentByPairFigiResponse>, Status> {
        let instrument_pair_figi = request.into_inner().instrument_pair_figi;
        let dbm = self.dbm.read().await;

        match dbm
            .read_instrument_by_pair_figi(&instrument_pair_figi)
            .await
        {
            Ok(instrument) => Ok(Response::new(get_instrument_by_pair_figi_response(
                instrument,
            ))),

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments(
        &self,
        _request: Request<GetAllInstrumentsRequest>,
    ) -> Result<Response<GetAllInstrumentsResponse>, Status> {
        self.dbg_print("get_all_instruments");

        let dbm = self.dbm.read().await;
        let res = dbm.read_all_instruments().await;

        match res {
            Ok(instruments) => Ok(Response::new(get_all_instruments_response(instruments))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_base_asset(
        &self,
        request: Request<GetAllInstrumentsForBaseAssetRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseAssetResponse>, Status> {
        self.dbg_print("get_all_instruments_for_base_asset");

        let base_asset = request.into_inner().base_asset;
        let dbm = self.dbm.read().await;
        let res = dbm.read_all_instruments_for_base_asset(&base_asset).await;

        match res {
            Ok(instruments) => Ok(Response::new(get_all_instruments_for_base_asset_response(
                instruments,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_quote_asset(
        &self,
        request: Request<GetAllInstrumentsForQuoteAssetRequest>,
    ) -> Result<Response<GetAllInstrumentsForQuoteAssetResponse>, Status> {
        self.dbg_print("get_all_instruments_for_quote_asset");

        let quote_asset = request.into_inner().quote_asset;
        let dbm = self.dbm.read().await;
        let res = dbm.read_all_instruments_for_quote_asset(&quote_asset).await;

        match res {
            Ok(instruments) => Ok(Response::new(get_all_instruments_for_quote_asset_response(
                instruments,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_exchange(
        &self,
        request: Request<GetAllInstrumentsForExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForExchangeResponse>, Status> {
        self.dbg_print("get_all_instruments_for_exchange");

        let exchange_code = request.into_inner().exchange_code;
        let dbm = self.dbm.read().await;
        let res = dbm.read_all_instruments_for_exchange(&exchange_code).await;

        match res {
            Ok(instruments) => Ok(Response::new(get_all_instruments_for_exchange_response(
                instruments,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_base_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForBaseAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseAssetAndExchangeResponse>, Status> {
        self.dbg_print("get_all_instruments_for_base_asset_and_exchange");

        let req = request.into_inner();
        let dbm = self.dbm.read().await;
        let res = dbm
            .read_all_instruments_for_base_asset_on_exchange(&req.base_asset, &req.exchange_code)
            .await;

        match res {
            Ok(instruments) => Ok(Response::new(
                get_all_instruments_for_base_asset_and_exchange_response(instruments),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_quote_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForQuoteAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForQuoteAssetAndExchangeResponse>, Status> {
        self.dbg_print("get_all_instruments_for_quote_asset_and_exchange");

        let req = request.into_inner();
        let dbm = self.dbm.read().await;
        let res = dbm
            .read_all_instruments_for_quote_asset_on_exchange(&req.quote_asset, &req.exchange_code)
            .await;

        match res {
            Ok(instruments) => Ok(Response::new(
                get_all_instruments_for_quote_asset_and_exchange_response(instruments),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_instruments_for_base_quote_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse>, Status> {
        self.dbg_print("get_all_instruments_for_base_quote_asset_and_exchange");

        let req = request.into_inner();
        let dbm = self.dbm.read().await;
        let res = dbm
            .read_all_instruments_for_base_quote_asset_on_exchange(
                &req.base_asset,
                &req.quote_asset,
                &req.exchange_code,
            )
            .await;

        match res {
            Ok(instruments) => Ok(Response::new(
                get_all_instruments_for_base_quote_asset_and_exchange_response(instruments),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn lookup_instrument_id_by_exchange_pair_code(
        &self,
        request: Request<LookupInstrumentIdByExchangePairCodeRequest>,
    ) -> Result<Response<LookupInstrumentIdByExchangePairCodeResponse>, Status> {
        self.dbg_print("lookup_instrument_id_by_exchange_pair_code");
        let req = request.into_inner();
        let dbm = self.dbm.read().await;

        match dbm
            .read_instruments_by_exchange_pair_code(&req.instrument_exchange_pair_code)
            .await
        {
            Ok(instrument) => Ok(Response::new(
                get_lookup_instrument_id_by_exchange_pair_code_response(instrument),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn lookup_instrument_id_by_figi(
        &self,
        request: Request<LookupInstrumentIdByFigiRequest>,
    ) -> Result<Response<LookupInstrumentIdByFigiResponse>, Status> {
        self.dbg_print("lookup_instrument_id_by_figi");
        let instrument_figi = request.into_inner().instrument_figi;
        let dbm = self.dbm.read().await;

        match dbm.read_instrument_by_figi(&instrument_figi).await {
            Ok(instrument) => Ok(Response::new(get_lookup_instrument_by_figi_response(
                instrument,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn lookup_instrument_id_by_pair_figi(
        &self,
        request: Request<LookupInstrumentIdByPairFigiRequest>,
    ) -> Result<Response<LookupInstrumentIdByPairFigiResponse>, Status> {
        self.dbg_print("lookup_instrument_id_by_figi");
        let instrument_pair_figi = request.into_inner().instrument_pair_figi;
        let dbm = self.dbm.read().await;

        match dbm
            .read_instrument_by_pair_figi(&instrument_pair_figi)
            .await
        {
            Ok(instrument) => Ok(Response::new(get_lookup_instrument_by_pair_figi_response(
                instrument,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
