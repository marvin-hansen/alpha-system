/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;

use std::error::Error;
use std::sync::Arc;
use trait_data_integration::ImsDataIntegration;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
///
/// The server manages message consumption and production for both control and data channels,
/// maintaining thread-safe access to shared resources using Tokio's async-aware locks.
pub struct Service<Integration: ImsDataIntegration> {
    dbg: bool,
    exchange_id: ExchangeID,
    ims_integration: Guarded<Integration>,
    integration_config: IntegrationConfig,
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub async fn build_service(
        dbg: bool,
        ims_integration: Integration,
        integration_config: &IntegrationConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(dbg, ims_integration, integration_config).await
    }
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    async fn build(
        dbg: bool,
        ims_integration: Integration,
        integration_config: &IntegrationConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[/Service]: {msg}");
            }
        };

        let exchange_id = integration_config.exchange_id();

        dbg_print("Create Service");
        Ok(Self {
            dbg,
            exchange_id,
            ims_integration: Arc::new(tokio::sync::RwLock::new(ims_integration)),
            integration_config: integration_config.clone(),
        })
    }
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[IMSData/Server]: {msg}");
        }
    }
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub(crate) async fn _shutdown(&self) -> Result<(), std::fmt::Error> {
        // let client_db = self.client_producers().read().await;
        //
        // if client_db.is_empty() {
        //     return Ok(());
        // }

        // self.dbg_print("Logging out all remaining clients");
        // for (client_id, _) in client_db.iter() {
        //     self.client_logout(*client_id)
        //         .await
        //         .unwrap_or_else(|_| panic!("Failed to log out client {client_id}"));
        // }

        self.dbg_print("Shutdown integration service");
        self.ims_integration
            .read()
            .await
            .shutdown()
            .await
            .expect("Failed to shutdown integration service");

        Ok(())
    }
}
