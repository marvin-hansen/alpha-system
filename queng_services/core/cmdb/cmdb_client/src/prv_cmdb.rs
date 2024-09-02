use common_exchange::prelude::PortfolioConfig;
use proto_cmdb::proto::{MultiPortfolioRequest, SinglePortfolioRequest};
use proto_utils::portfolio_proto_utils::{portfolio_config_from_proto, portfolio_config_to_proto};

use crate::{CMDBError, CmdbClient};

impl CmdbClient {
    /// Asynchronously creates a portfolio configuration.
    ///
    /// # Arguments
    ///
    /// * `data` - The portfolio configuration to create.
    ///
    /// # Returns
    ///
    /// A `Result` that is either `Ok(true)` if the portfolio configuration was successfully created,
    /// or an `Err` containing a `CMDBError` if there was an error creating the portfolio configuration.
    ///
    pub async fn create_portfolio_config(self, data: PortfolioConfig) -> Result<bool, CMDBError> {
        let proto_portfolio_config = portfolio_config_to_proto(data)
            .expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_portfolio_config);

        let mut client = self.client.clone();

        match client.create_portfolio_config(request).await {
            Ok(res) => Ok(res.into_inner().portfolio_created),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    /// Asynchronously reads a portfolio configuration by its ID.
    ///
    /// # Arguments
    ///
    /// * `self` - The `CmdbManager` instance.
    /// * `id` - The ID of the portfolio configuration to read.
    ///
    /// # Returns
    ///
    /// A `Result` that is either `Ok(Some(PortfolioConfig))` if the portfolio configuration was successfully read,
    /// or `Ok(None)` if the portfolio configuration does not exist, or an `Err` containing a `CMDBError` if there was an
    /// error reading the portfolio configuration.
    pub async fn read_portfolio_config_by_id(
        self,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, CMDBError> {
        let request = tonic::Request::new(SinglePortfolioRequest {
            portfolio_id: id as u32,
        });

        let mut client = self.client.clone();

        match client.read_portfolio_config(request).await {
            Ok(res) => match res.into_inner().portfolio_config {
                Some(p) => Ok(Some(portfolio_config_from_proto(p.to_owned()).expect(
                    "Failed to convert ProtoPortfolioConfig to Rust PortfolioConfig",
                ))),
                None => Ok(None),
            },
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    /// Asynchronously reads all portfolio configurations.
    ///
    /// # Arguments
    ///
    /// * `self` - The `CmdbManager` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `PortfolioConfig` if the portfolio configurations were successfully read,
    /// or an `Err` containing a `CMDBError` if there was an error reading the portfolio configurations.
    pub async fn read_all_portfolio_configs(self) -> Result<Vec<PortfolioConfig>, CMDBError> {
        let request = tonic::Request::new(MultiPortfolioRequest {
            portfolios_all: true,
        });

        let mut client = self.client.clone();

        match client.read_all_portfolio_configs(request).await {
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
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    /// Asynchronously updates a portfolio configuration.
    ///
    /// # Arguments
    ///
    /// * `self` - The `CmdbManager` instance.
    /// * `data` - The updated portfolio configuration.
    ///
    /// # Returns
    ///
    /// A `Result` that is either `Ok(true)` if the portfolio configuration was successfully updated,
    /// or an `Err` containing a `CMDBError` if there was an error updating the portfolio configuration.
    ///
    pub async fn update_portfolio_config(self, data: PortfolioConfig) -> Result<bool, CMDBError> {
        let proto_portfolio_config = portfolio_config_to_proto(data)
            .expect("Failed to convert Rust PortfolioConfig to proto");

        let request = tonic::Request::new(proto_portfolio_config);

        let mut client = self.client.clone();

        match client.update_portfolio_config(request).await {
            Ok(res) => Ok(res.into_inner().portfolio_updated),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    /// Asynchronously deletes a portfolio configuration.
    ///
    /// # Arguments
    ///
    /// * `self` - The `CmdbManager` instance.
    /// * `id` - The ID of the portfolio configuration to delete.
    ///
    /// # Returns
    ///
    /// A `Result` that is either `Ok(true)` if the portfolio configuration was successfully deleted,
    /// or an `Err` containing a `CMDBError` if there was an error deleting the portfolio configuration.
    pub async fn delete_portfolio_config(self, id: u16) -> Result<bool, CMDBError> {
        let request = tonic::Request::new(SinglePortfolioRequest {
            portfolio_id: id as u32,
        });

        let mut client = self.client.clone();

        match client.delete_portfolio_config(request).await {
            Ok(res) => Ok(res.into_inner().portfolio_deleted),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }
}
