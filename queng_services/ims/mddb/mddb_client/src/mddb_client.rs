use crate::error::MDDBClientError;
use crate::MDDBClient;
use common_metadata::prelude::MetaAsset;
use proto_mddb::proto::{
    CheckIfExchangeIdExistsRequest, CheckIfExchangeIdExistsResponse,
    CheckIfInstrumentIdExistsRequest, CheckIfInstrumentIdExistsResponse, CountExchangesRequest,
    CountExchangesResponse, CountInstrumentsRequest, CountInstrumentsResponse,
    GetAllExchangesRequest, GetAllExchangesResponse,
    GetAllInstrumentsForBaseAssetAndExchangeRequest,
    GetAllInstrumentsForBaseAssetAndExchangeResponse, GetAllInstrumentsForBaseAssetRequest,
    GetAllInstrumentsForBaseAssetResponse, GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest,
    GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse, GetAllInstrumentsForExchangeRequest,
    GetAllInstrumentsForExchangeResponse, GetAllInstrumentsForQuoteAssetAndExchangeRequest,
    GetAllInstrumentsForQuoteAssetAndExchangeResponse, GetAllInstrumentsForQuoteAssetRequest,
    GetAllInstrumentsForQuoteAssetResponse, GetAllInstrumentsRequest, GetAllInstrumentsResponse,
    GetExchangeRequest, GetExchangeResponse, GetInstrumentByFigiRequest, GetInstrumentByIdRequest,
    GetInstrumentByIdResponse, LookupExchangeNameRequest, LookupExchangeNameResponse,
    LookupInstrumentExchangePairCodeRequest, LookupInstrumentFigiRequest,
    LookupInstrumentFigiResponse, ProtoMetaAsset,
};
use proto_mddb_utils::mddb_assets_utils as assets_utils;
use tonic::{Request, Response, Status};

impl MDDBClient {
    async fn count_assets(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_count_assets_request();

        match client.count_assets(request).await {
            Ok(res) => Ok(res.into_inner().count),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn check_if_asset_id_exists(&self, asset_id: &str) -> Result<bool, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_check_if_asset_exists_request(asset_id);

        match client.check_if_asset_id_exists(request).await {
            Ok(res) => Ok(res.get_ref().exists),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn get_asset(&self, asset_id: &str) -> Result<Option<MetaAsset>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_asset_request(asset_id);

        match client.get_asset(request).await {
            Ok(res) => Ok(res
                .into_inner()
                .asset
                .map(|asset| assets_utils::proto_asset_to_meta_asset(&asset))),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn get_all_assets(&self) -> Result<Vec<MetaAsset>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_all_assets_request();

        match client.get_all_assets(request).await {
            Ok(res) => {
                let assets = res
                    .into_inner()
                    .assets
                    .into_iter()
                    .map(|proto_asset: ProtoMetaAsset| {
                        assets_utils::proto_asset_to_meta_asset(&proto_asset)
                    })
                    .collect();

                Ok(assets)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn count_exchanges(
        &self,
        request: Request<CountExchangesRequest>,
    ) -> Result<Response<CountExchangesResponse>, Status> {
        todo!()
    }

    async fn check_if_exchange_id_exists(
        &self,
        request: Request<CheckIfExchangeIdExistsRequest>,
    ) -> Result<Response<CheckIfExchangeIdExistsResponse>, Status> {
        todo!()
    }

    async fn get_exchange(
        &self,
        request: Request<GetExchangeRequest>,
    ) -> Result<Response<GetExchangeResponse>, Status> {
        todo!()
    }

    async fn get_all_exchanges(
        &self,
        request: Request<GetAllExchangesRequest>,
    ) -> Result<Response<GetAllExchangesResponse>, Status> {
        todo!()
    }

    async fn lookup_exchange_name(
        &self,
        request: Request<LookupExchangeNameRequest>,
    ) -> Result<Response<LookupExchangeNameResponse>, Status> {
        todo!()
    }

    async fn count_instruments(
        &self,
        request: Request<CountInstrumentsRequest>,
    ) -> Result<Response<CountInstrumentsResponse>, Status> {
        todo!()
    }

    async fn check_if_instrument_id_exists(
        &self,
        request: Request<CheckIfInstrumentIdExistsRequest>,
    ) -> Result<Response<CheckIfInstrumentIdExistsResponse>, Status> {
        todo!()
    }

    async fn get_instrument(
        &self,
        request: Request<GetInstrumentByIdRequest>,
    ) -> Result<Response<GetInstrumentByIdResponse>, Status> {
        todo!()
    }

    async fn get_instrument_by_figi(
        &self,
        request: Request<GetInstrumentByFigiRequest>,
    ) -> Result<Response<GetInstrumentByIdResponse>, Status> {
        todo!()
    }

    async fn get_all_instruments(
        &self,
        request: Request<GetAllInstrumentsRequest>,
    ) -> Result<Response<GetAllInstrumentsResponse>, Status> {
        todo!()
    }

    async fn get_all_instruments_for_base_asset(
        &self,
        request: Request<GetAllInstrumentsForBaseAssetRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseAssetResponse>, Status> {
        todo!()
    }

    async fn get_all_instruments_for_quote_asset(
        &self,
        request: Request<GetAllInstrumentsForQuoteAssetRequest>,
    ) -> Result<Response<GetAllInstrumentsForQuoteAssetResponse>, Status> {
        todo!()
    }

    async fn get_all_instruments_for_exchange(
        &self,
        request: Request<GetAllInstrumentsForExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForExchangeResponse>, Status> {
        todo!()
    }

    async fn get_all_instruments_for_base_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForBaseAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseAssetAndExchangeResponse>, Status> {
        todo!()
    }

    async fn get_all_instruments_for_quote_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForQuoteAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForQuoteAssetAndExchangeResponse>, Status> {
        todo!()
    }

    async fn get_all_instruments_for_base_quote_asset_and_exchange(
        &self,
        request: Request<GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest>,
    ) -> Result<Response<GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse>, Status> {
        todo!()
    }

    async fn lookup_instrument_exchange_pair_code_name(
        &self,
        request: Request<LookupInstrumentExchangePairCodeRequest>,
    ) -> Result<Response<LookupInstrumentExchangePairCodeRequest>, Status> {
        todo!()
    }

    async fn lookup_instrument_figi(
        &self,
        request: Request<LookupInstrumentFigiRequest>,
    ) -> Result<Response<LookupInstrumentFigiResponse>, Status> {
        todo!()
    }
}
