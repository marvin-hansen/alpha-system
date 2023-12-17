use common::prelude::DataBar;
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
    /// # Example
    ///
    /// ```
    /// use file_manager::FileManager;
    /// let file_manager = FileManager::new();
    /// let bars = file_manager.read_data_from_file("data.parquet")?;
    /// ```
    pub fn read_data_from_file(path: &str) -> Result<Vec<DataBar>, Box<dyn Error>> {
        if false == Path::new(path).exists() {
            return Err(Box::try_from(format!("File {} does not exist", path)).unwrap());
        }

        read_parquet(&path)
    }
}

fn read_parquet(path: &str) -> Result<Vec<DataBar>, Box<dyn Error>> {
    let mut content: Vec<DataBar> = Vec::with_capacity(1500); // fixed pre-allocation

    let file = File::open(&Path::new(path)).expect("Could not open file");

    let reader = SerializedFileReader::new(file).expect("Could not create parquet reader");

    let mut iter = reader
        .get_row_iter(None)
        .expect("Could not create parquet row iterator");

    while let Some(record) = iter.next() {
        let record = record.expect("Could not read record");
        let bar = match convert_field_to_bar(&record) {
            Ok(bar) => bar,
            Err(e) => panic!("Could not convert field to bar: {}", e),
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
/// - 0: date_time String
/// - 1: symbol String
/// - 2: open f64
/// - 3: high f64
/// - 4: low f64
/// - 5: close f64
/// - 6: volume f64
/// - 7: trades u64
///
fn convert_field_to_bar(record: &Row) -> Result<DataBar, Box<dyn Error>> {
    // parquet index.
    // 0 date_time String
    // 1 symbol String
    // 2 open f64
    // 3 high f64
    // 4 low f64
    // 5 close f64
    // 6 volume f64
    // 7 trades u64
    // We can safely unwrap b/c all data are complete and correct.
    let date_time = record.get_string(0).unwrap().to_owned();
    let symbol = record.get_string(1).unwrap().to_owned();
    let open = Decimal::from_f64(record.get_double(2).unwrap()).unwrap();
    let high = Decimal::from_f64(record.get_double(3).unwrap()).unwrap();
    let low = Decimal::from_f64(record.get_double(4).unwrap()).unwrap();
    let close = Decimal::from_f64(record.get_double(5).unwrap()).unwrap();
    let volume = Decimal::from_f64(record.get_double(6).unwrap()).unwrap();
    let trades = Decimal::from(record.get_ulong(7).unwrap());

    let bar = DataBar::new(date_time, symbol, open, high, low, close, volume, trades);

    Ok(bar)
}
