use db_utils::fields::INACTIVE_EXCHANGES;
use db_utils::prelude::ClickHouseClient;
use db_utils::types::Instrument;
use db_utils::{insert, query_utils};
use std::error::Error;

pub(crate) async fn import_instruments(
    client: &ClickHouseClient,
    instruments: &Vec<Instrument>,
) -> Result<(), Box<dyn Error>> {
    for instrument in instruments.iter() {
        // Skip all instruments from inactive exchanges
        if INACTIVE_EXCHANGES.contains(&instrument.exchange_code()) {
            continue;
        }

        if is_valid_instrument(instrument) {
            let insert_query = insert::generate_instruments_insert(instrument);
            query_utils::execute_query(client, &insert_query)
                .await
                .expect("Failed to insert asset")
        }
    }

    Ok(())
}

// Double check if instrument is inactive i.e. from an inactive exchange
fn is_valid_instrument(instrument: &Instrument) -> bool {
    // Instrument  inactive
    if instrument.trade_start_time.is_none() && instrument.trade_end_time.is_none() {
        return false;
    }

    // Instrument inactive
    if instrument.trade_end_time.is_some() && instrument.trade_end_timestamp.is_some() {
        return false;
    }

    // Instrument is of no interest
    if instrument.class.eq("option") {
        return false;
    }

    // Non-perpetual future contracts.
    if instrument.class.eq("future") {
        return false;
    }

    true
}
