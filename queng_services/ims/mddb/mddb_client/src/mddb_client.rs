use crate::error::MDDBClientError;
use crate::MDDBClient;
use common_metadata::prelude::{MetaAsset, MetaExchange, MetaInstrument};
use proto_mddb::proto::*;
use proto_mddb_utils::mddb_assets_utils as assets_utils;
use proto_mddb_utils::mddb_exchanges_utils as exchanges_utils;

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

    async fn count_exchanges(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = exchanges_utils::get_count_exchanges_request();

        match client.count_exchanges(request).await {
            Ok(res) => Ok(res.into_inner().count),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn check_if_exchange_id_exists(
        &self,
        exchange_code: &str,
    ) -> Result<bool, MDDBClientError> {
        let mut client = self.client.clone();
        let request = exchanges_utils::get_check_if_exchange_exists_request(exchange_code);

        match client.check_if_exchange_id_exists(request).await {
            Ok(res) => Ok(res.get_ref().exists),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn get_exchange(&self, exchange_code: &str) -> Result<MetaExchange, MDDBClientError> {
        let mut client = self.client.clone();
        let request = exchanges_utils::get_exchange_request(exchange_code);

        match client.get_exchange(request).await {
            Ok(res) => {
                let exchange = res
                    .into_inner()
                    .exchange
                    .map(|exchange| exchanges_utils::proto_exchange_to_meta_exchange(&exchange))
                    .ok_or_else(|| MDDBClientError("Exchange not found".to_string()))?;
                Ok(exchange)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn get_all_exchanges(&self) -> Result<Vec<MetaExchange>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = exchanges_utils::get_all_exchanges_request();

        match client.get_all_exchanges(request).await {
            Ok(res) => {
                let exchanges = res
                    .into_inner()
                    .exchanges
                    .into_iter()
                    .map(|proto_exchange| {
                        exchanges_utils::proto_exchange_to_meta_exchange(&proto_exchange)
                    })
                    .collect();
                Ok(exchanges)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn lookup_exchange_name(&self, exchange_code: &str) -> Result<String, MDDBClientError> {
        let mut client = self.client.clone();
        let request = exchanges_utils::get_lookup_exchange_name_request(exchange_code);

        match client.lookup_exchange_name(request).await {
            Ok(res) => Ok(res.into_inner().exchange_name),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    async fn count_instruments(&self) -> Result<u64, MDDBClientError> {
        todo!()
    }

    async fn check_if_instrument_id_exists(
        &self,
        instrument_id: &str,
    ) -> Result<bool, MDDBClientError> {
        todo!()
    }

    async fn get_instrument(&self, instrument_id: &str) -> Result<MetaInstrument, MDDBClientError> {
        todo!()
    }

    async fn get_instrument_by_figi(
        &self,
        instrument_figi: &str,
    ) -> Result<Option<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn get_all_instruments(
        &self,
        instrument_figi: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn get_all_instruments_for_base_asset(
        &self,
        base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn get_all_instruments_for_quote_asset(
        &self,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn get_all_instruments_for_exchange(
        &self,
        exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn get_all_instruments_for_base_asset_and_exchange(
        &self,
        exchange_code: &str,
        base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn get_all_instruments_for_quote_asset_and_exchange(
        &self,
        exchange_code: &str,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn get_all_instruments_for_base_quote_asset_and_exchange(
        &self,
        exchange_code: &str,
        base_asset: &str,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        todo!()
    }

    async fn lookup_instrument_exchange_pair_code_name(
        &self,
        instrument_id: &str,
    ) -> Result<String, MDDBClientError> {
        todo!()
    }

    async fn lookup_instrument_figi(
        &self,
        instrument_id: &str,
    ) -> Result<Option<String>, MDDBClientError> {
        todo!()
    }
}
