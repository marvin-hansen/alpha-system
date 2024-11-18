use crate::DBGWCmdbClient;
use common_errors::DBGatewayError;
use common_exchange::PortfolioConfig;
use proto_cmdb::proto::{MultiPortfolioRequest, SinglePortfolioRequest};
use proto_cmdb_utils::portfolio_proto_utils::{
    portfolio_config_from_proto, portfolio_config_to_proto,
};

impl DBGWCmdbClient {
    pub async fn create_portfolio_config(
        mut self,
        data: PortfolioConfig,
    ) -> Result<bool, DBGatewayError> {
        //
        let proto_portfolio_config = portfolio_config_to_proto(data)
            .expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_portfolio_config);

        let res = self.client.create_portfolio_config(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().portfolio_created),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn read_all_portfolio_configs(
        mut self,
    ) -> Result<Vec<PortfolioConfig>, DBGatewayError> {
        let request = tonic::Request::new(MultiPortfolioRequest {
            portfolios_all: true,
        });

        let res = self.client.read_all_portfolio_configs(request).await;

        match res {
            Ok(res) => {
                let vec = res
                    .into_inner()
                    .portfolio_configs
                    .iter()
                    .map(|p| {
                        portfolio_config_from_proto(p.to_owned()).expect(
                            "Failed to convert ProtoPortfolioConfig to Rust PortfolioConfig",
                        )
                    })
                    .collect::<Vec<PortfolioConfig>>();

                Ok(vec)
            }
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn read_portfolio_config_by_id(
        mut self,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, DBGatewayError> {
        let request = tonic::Request::new(SinglePortfolioRequest {
            portfolio_id: id as u32,
        });

        let res = self.client.read_portfolio_config(request).await;

        match res {
            Ok(res) => match res.into_inner().portfolio_config {
                Some(p) => Ok(Some(portfolio_config_from_proto(p.to_owned()).expect(
                    "Failed to convert ProtoPortfolioConfig to Rust PortfolioConfig",
                ))),
                None => Ok(None),
            },
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn update_portfolio_config(
        mut self,
        data: PortfolioConfig,
    ) -> Result<bool, DBGatewayError> {
        let proto_portfolio_config = portfolio_config_to_proto(data)
            .expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_portfolio_config);

        let res = self.client.update_portfolio_config(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().portfolio_updated),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    pub async fn delete_portfolio_config(mut self, id: u16) -> Result<bool, DBGatewayError> {
        let request = tonic::Request::new(SinglePortfolioRequest {
            portfolio_id: id as u32,
        });

        let res = self.client.delete_portfolio_config(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().portfolio_deleted),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }
}
