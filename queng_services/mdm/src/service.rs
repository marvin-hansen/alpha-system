use crate::client::binance::binance_client::BinanceRESTClient;
use std::fmt::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type Guarded<T> = Arc<RwLock<T>>;

pub struct MetaDataServer {
    reference_symbols: Guarded<Vec<String>>,
    rest_client: Guarded<BinanceRESTClient>,
}

impl MetaDataServer {
    pub fn new() -> Self {
        let client = BinanceRESTClient::new().expect("Failed to build Binance REST client");
        let rest_client = Arc::new(RwLock::new(client));

        Self {
            reference_symbols: Arc::new(RwLock::new(Vec::new())),
            rest_client,
        }
    }
}

impl MetaDataServer {
    pub async fn reference_symbols(&self) -> Vec<String> {
        let g = &self.reference_symbols.read().await;
        g.to_vec()
    }
}

impl MetaDataServer {
    pub async fn update_reference_symbols(&self) -> Result<(), Error> {
        let client = self.rest_client.read().await;
        let reference_symbols = client
            .get_available_symbols()
            .await
            .expect("Failed to get reference symbols from Binance");

        let mut guard = self.reference_symbols.write().await;
        *guard = reference_symbols;
        drop(guard);

        Ok(())
    }

    pub async fn validate_symbols(&self, symbols: &Vec<String>) -> Result<(), String> {
        let guard = self.reference_symbols.read().await;

        if guard.to_vec().len() == 0 {
            self.update_reference_symbols()
                .await
                .expect("Failed to update reference symbols");
        }

        for symbol in symbols {
            if !guard.to_vec().contains(symbol) {
                return Err(symbol.to_string());
            }
        }

        drop(guard);

        Ok(())
    }
}
