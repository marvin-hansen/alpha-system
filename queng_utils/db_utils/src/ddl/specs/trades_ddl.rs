pub fn generate_trades_table_ddl(table_name: &str) -> String {
    format!(
        r"
        CREATE TABLE IF NOT EXISTS default.{table_name}
        (
           `timestamp` Datetime64(3) CODEC(Delta(4), ZSTD(1)),
           `price`  Float64 CODEC(Delta, LZ4),
           `volume` Float64 CODEC(Delta, LZ4),
        )
         ENGINE = MergeTree()
         PRIMARY KEY toStartOfHour(timestamp)
     "
    )
}
