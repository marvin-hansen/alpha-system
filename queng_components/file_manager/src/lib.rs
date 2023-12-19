use chrono::{DateTime, TimeZone, Utc};
use common::prelude::{DataBar, FileConfig, SymbolID};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileManager {}

impl FileManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileManager {
    /// Reads parquet file at given path and converts rows to Vec<DataBar>
    ///
    /// # Parameters
    ///
    /// * `path` - Path to parquet file as &str
    ///
    /// # Returns
    ///
    /// Result<Vec<DataBar>, Box<dyn Error>> - Vector of DataBar structs parsed
    /// from file, or error if file read/parse fails
    ///
    pub fn read_data_from_file(
        &self,
        file_config: &FileConfig,
    ) -> Result<Vec<DataBar>, Box<dyn Error>> {
        let path = file_config.path();
        if false == Path::new(path).exists() {
            return Err(
                Box::try_from(format!("[FileManager]: File {} does not exist", path)).unwrap(),
            );
        }

        read_parquet(file_config)
    }
}

fn read_parquet(file_config: &FileConfig) -> Result<Vec<DataBar>, Box<dyn Error>> {
    let path = file_config.path();

    let file = File::open(&Path::new(path)).expect("[FileManager]: Could not open file");

    let symbol = file_config.data_symbol();

    let mut content: Vec<DataBar> = Vec::with_capacity(1500); // fixed pre-allocation

    let reader =
        SerializedFileReader::new(file).expect("[FileManager]: Could not create parquet reader");

    let mut iter = reader
        .get_row_iter(None)
        .expect("[FileManager]: Could not create parquet row iterator");

    while let Some(record) = iter.next() {
        let record = record.expect("[FileManager]: Could not read record");
        let bar = match convert_field_to_bar(&record, symbol) {
            Ok(bar) => bar,
            Err(e) => panic!("[FileManager]: Could not convert field to bar: {}", e),
        };

        content.push(bar);
    }

    Ok(content)
}

/// Converts a parquet Row to a DataBar
///
/// # Parameters
///
/// * `record` - Parquet Row to convert
///
/// # Returns
///
/// Result<DataBar, Box<dyn Error>> - DataBar parsed from the row, or an error
///
/// # Remarks
///
/// This makes assumptions about the schema based on position:
///
/// 0 date_time String
/// 1 open f64
/// 2 high f64
/// 3 low f64
/// 4 close f64
/// 5 volume f64
///
fn convert_field_to_bar(row: &Row, symbol: SymbolID) -> Result<DataBar, Box<dyn Error>> {
    // parquet index.
    // 0 date_time String
    // 1 open f64
    // 2 high f64
    // 3 low f64
    // 4 close f64
    // 5 volume f64

    // Extract fields from row.
    let date_time: DateTime<Utc> =
        get_date_time_field(row).expect("[FileManager]: Failed to get date_time field");
    let open_price: f64 = row
        .get_double(1)
        .expect("[FileManager]: Cannot extract open price");
    let high_price: f64 = row
        .get_double(2)
        .expect("[FileManager]: Cannot extract high price");
    let low_price: f64 = row
        .get_double(3)
        .expect("[FileManager]: Cannot extract low price");
    let close_price: f64 = row
        .get_double(4)
        .expect("[FileManager]: Cannot extract close price");
    let volume: f64 = row
        .get_double(5)
        .expect("[FileManager]: Cannot extract close price");

    // Convert fields to Rust types.
    let open = Decimal::from_f64(open_price).expect("[FileManager]: Failed to parse open price");
    let high = Decimal::from_f64(high_price).expect("[FileManager]: Failed to parse high price");
    let low = Decimal::from_f64(low_price).expect("[FileManager]: Failed to parse low price");
    let close = Decimal::from_f64(close_price).expect("[FileManager]: Failed to parse close price");
    let volume = Decimal::from_f64(volume).expect("[FileManager]: Failed to parse volume");

    // Build DataBar.
    let bar = DataBar::new(date_time, symbol, open, high, low, close, volume);

    Ok(bar)
}

fn get_date_time_field(row: &Row) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    if row.get_string(0).is_ok() {
        // supported timezone syntax for DateTime from string https://github.com/chronotope/chrono/issues/219
        let fmt = "%Y-%m-%d %H:%M:%S%.6f%z";
        let s = row.get_string(0).expect("Cannot extract datetime str");
        // supported timezone syntax for DateTime from string https://github.com/chronotope/chrono/issues/219
        let date_time: DateTime<Utc> = DateTime::parse_from_str(s, fmt)
            .expect("Cannot convert string to DateTime")
            .with_timezone(&Utc);

        return Ok(date_time);
    }

    if row.get_long(0).is_ok() {
        let millis = row.get_long(0).expect("Cannot extract datetime millis");
        let date_time: DateTime<Utc> = Utc.timestamp_millis_opt(millis).unwrap();
        return Ok(date_time);
    }

    if row.get_timestamp_micros(0).is_ok() {
        let micros = row
            .get_timestamp_micros(0)
            .expect("Cannot extract datetime millis");
        let millis = micros / 1000;

        let date_time: DateTime<Utc> = Utc.timestamp_millis_opt(millis).unwrap();
        return Ok(date_time);
    }

    panic!("get_date_time_field: Cannot extract datetime field");
}
