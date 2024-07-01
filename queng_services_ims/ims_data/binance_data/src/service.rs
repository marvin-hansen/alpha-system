use crate::stream_manager::stream_manager;
use crate::types::command::DataCommand;
use common::prelude::DataType;
use proto_bindings::proto::ims_data_service_server::ImsDataService;
use proto_bindings::proto::*;
use std::sync::atomic;
use std::sync::atomic::AtomicU32;
use tokio::sync::mpsc::Sender;
use tonic::{Request, Response, Status};

// Binance docs states that at most a connection can handle 1024 streams.
// In practice, its more close to 950 because of the URL length limit but we
// settle for 800 leaving plenty of room for long stream names.
const MAX_SYMBOLS: usize = 800;
const ORDER: atomic::Ordering = atomic::Ordering::Relaxed;

pub struct ImsDataServer {
    counter: AtomicU32,
    tx: Sender<DataCommand>,
}

impl ImsDataServer {
    pub fn new() -> Self {
        let counter = AtomicU32::new(1);
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(stream_manager(rx));
        Self { counter, tx }
    }
}

#[tonic::async_trait]
impl ImsDataService for ImsDataServer {
    async fn start_data(
        &self,
        request: Request<ProtoStartDataRequest>,
    ) -> Result<Response<ProtoStartDataResponse>, Status> {
        println!("[ImsDataBinance/start_data]",);

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

        let stream_id = self.counter.fetch_add(1, ORDER);
        let exchange_id = req.exchange_id;

        let symbols = req.symbols;
        let data_type = DataType::from(req.data_type_id as u8);

        println!("[ImsDataBinance/start_data]: Starting data stream...");
        let start_command = DataCommand::Start(stream_id, symbols, data_type);
        self.tx
            .send(start_command)
            .await
            .expect("Failed to send start data command");
        //
        println!("[ImsDataBinance/start_data]: Data started");

        Ok(Response::new(ProtoStartDataResponse {
            exchange_id,
            stream_id,
        }))
    }

    async fn stop_data(
        &self,
        request: Request<ProtoStopDataRequest>,
    ) -> Result<Response<ProtoStopDataResponse>, Status> {
        println!("[ImsDataBinance/stop_data]");

        let req = request.into_inner();

        let stream_id = req.stream_id;
        let exchange_id = req.exchange_id;

        println!("[ImsDataBinance/StopData]: Stopping data stream...");
        let stop_command = DataCommand::Stop(stream_id);
        self.tx
            .send(stop_command)
            .await
            .expect("Failed to send start data command");
        //
        println!("[ImsDataBinance/stop_data]: Data stream stopped");

        Ok(Response::new(ProtoStopDataResponse {
            exchange_id,
            ok: true,
        }))
    }

    async fn stop_all_data(
        &self,
        request: Request<ProtoStopAllDataRequest>,
    ) -> Result<Response<ProtoStopAllDataResponse>, Status> {
        println!("[ImsDataBinance/stop_all_data]");

        let req = request.into_inner();
        let exchange_id = req.exchange_id;

        println!("[ImsDataBinance/StopAllData]: Stopping all data streams...");
        let stop_all_command = DataCommand::StopAll;
        self.tx
            .send(stop_all_command)
            .await
            .expect("Failed to send start data command");
        //
        println!("[ImsDataBinance/stop_all_data]: All data streams stopped");

        Ok(Response::new(ProtoStopAllDataResponse {
            exchange_id,
            ok: true,
        }))
    }
}
