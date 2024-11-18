use crate::stream_manager::stream_manager;
use common_data_bar::DataType;
use ims_common::prelude::BinanceDataCommand;
use std::sync::atomic;
use std::sync::atomic::AtomicU32;
use tokio::sync::mpsc::Sender;
use tonic::{Request, Status};

// Binance docs states that at most a connection can handle 1024 streams.
// In practice, its more close to 950 because of the URL length limit but we
// settle for 800 leaving plenty of room for long stream names.
const MAX_SYMBOLS: usize = 800;
const ORDER: atomic::Ordering = atomic::Ordering::Relaxed;

pub struct ImsDataServer {
    counter: AtomicU32,
    tx: Sender<BinanceDataCommand>,
}

impl ImsDataServer {
    pub fn new() -> Self {
        let counter = AtomicU32::new(1);
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(stream_manager(rx));
        Self { counter, tx }
    }
}

impl ImsDataServer {
    async fn start_data(
        &self,
        request: Request<(u32, Vec<String>, DataType)>,
    ) -> Result<(u32, u32), Status> {
        println!("[ImsDataBinance/start_data]",);

        let req = request.into_inner();
        let nr_symbols = req.1.capacity();
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
        let exchange_id = req.0;

        let symbols = req.1;
        let data_type = req.2;

        println!("[ImsDataBinance/start_data]: Starting data stream...");
        let start_command = BinanceDataCommand::Start(stream_id, symbols, data_type);
        self.tx
            .send(start_command)
            .await
            .expect("Failed to send start data command");
        //
        println!("[ImsDataBinance/start_data]: Data started");

        Ok((exchange_id, stream_id))
    }

    async fn stop_data(&self, request: Request<(u32, u32)>) -> Result<(u32, bool), Status> {
        println!("[ImsDataBinance/stop_data]");

        let req = request.into_inner();

        let stream_id = req.0;
        let exchange_id = req.1;

        println!("[ImsDataBinance/StopData]: Stopping data stream...");
        let stop_command = BinanceDataCommand::Stop(stream_id);
        self.tx
            .send(stop_command)
            .await
            .expect("Failed to send start data command");
        //
        println!("[ImsDataBinance/stop_data]: Data stream stopped");

        Ok((exchange_id, true))
    }

    async fn stop_all_data(&self, request: Request<u32>) -> Result<(u32, bool), Status> {
        println!("[ImsDataBinance/stop_all_data]");

        let req = request.into_inner();
        let exchange_id = req;

        println!("[ImsDataBinance/StopAllData]: Stopping all data streams...");
        let stop_all_command = BinanceDataCommand::StopAll;
        self.tx
            .send(stop_all_command)
            .await
            .expect("Failed to send start data command");
        //
        println!("[ImsDataBinance/stop_all_data]: All data streams stopped");

        Ok((exchange_id, true))
    }
}
