use crate::client::binance_client::BinanceRESTClient;
use crate::stream_manager::stream_manager;
use crate::types::alias::Guarded;
use crate::types::command::Command;
use common::prelude::DataType;
use proto::binding::ims_data_service_server::ImsDataService;
use proto::binding::*;
use std::fmt::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tonic::{Request, Response, Status};

// Binance docs states that at most a connection can handle 1024 streams.
// In practice, its more close to 950 because of the URL length limit but we
// settle for 800 to leave plenty of room for long stream names.
const MAX_SYMBOLS: usize = 800;

pub struct ImsDataServer {
    rest_client: Guarded<BinanceRESTClient>,
    reference_symbols: Guarded<Vec<String>>,
    handle: JoinHandle<()>,
    tx: Sender<Command>,
}

impl ImsDataServer {
    pub fn new() -> Self {
        let client = BinanceRESTClient::new().expect("Failed to build Binance REST client");
        let rest_client = Arc::new(RwLock::new(client));
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let handle = tokio::spawn(stream_manager(rx));

        Self {
            rest_client,
            reference_symbols: Arc::new(RwLock::new(Vec::new())),
            handle,
            tx,
        }
    }
}

impl ImsDataServer {
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

        if guard.len() == 0 {
            self.update_reference_symbols()
                .await
                .expect("Failed to update reference symbols");
        }

        for symbol in symbols {
            if !guard.contains(symbol) {
                return Err(symbol.to_string());
            }
        }

        drop(guard);

        Ok(())
    }
}

#[tonic::async_trait]
// #[autometrics]
impl ImsDataService for ImsDataServer {
    async fn start_data(
        &self,
        request: Request<ProtoStartDataRequest>,
    ) -> Result<Response<ProtoStartDataResponse>, Status> {
        println!(
            "[ImsDataBinance/start_data]: Processing request, {:?}",
            request
        );

        let req = request.into_inner();
        let nr_symbols = req.symbols.capacity();
        if nr_symbols == 0 {
            return Err(Status::invalid_argument("No symbols specified"));
        }

        if nr_symbols > MAX_SYMBOLS {
            return Err(Status::invalid_argument(format!(
                "Requested number of symbols ({}) exceeds maximum allowed number of symbols ({})",
                nr_symbols, MAX_SYMBOLS
            )));
        }

        let symbols = match self.validate_symbols(&req.symbols).await {
            Ok(_) => req.symbols,
            Err(e) => return Err(Status::invalid_argument(format!("Invalid symbol: {}", e))),
        };

        let data_type = DataType::from(req.data_type_id as u8);

        let command = Command::StartData(symbols, data_type);

        self.tx
            .send(command)
            .await
            .expect("Failed to send start data command");
        println!("[ImsDataBinance]: Data started");

        Ok(Response::new(ProtoStartDataResponse {
            exchange_id: req.exchange_id,
            data_type_id: req.data_type_id,
            data_started: true,
        }))
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
