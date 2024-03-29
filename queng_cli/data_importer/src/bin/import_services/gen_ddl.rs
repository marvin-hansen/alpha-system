// Tuple type cannot contain codecs on types,
// https://clickhouse.com/docs/en/sql-reference/data-types/tuple
pub(crate) fn generate_services_table_ddl(table_name: &str) -> String {
    format!(
        r"
    CREATE TABLE IF NOT EXISTS {table_name}
    (
        `svc_id` String CODEC(LZ4),
        `name` String CODEC(LZ4),
        `version` UInt8 CODEC(LZ4),
        `online` Bool CODEC(LZ4),
        `description` String CODEC(LZ4),
        `health_check_uri` String CODEC(LZ4),
        `base_uri` LowCardinality(String) CODEC(LZ4),
        `dependencies` Array(Nullable(String)) CODEC(LZ4),
        `exposure`  LowCardinality(String) CODEC(LZ4),
        `endpoint`  Tuple(
            `name` String,
            `version` UInt8,
            `description` String,
            `uri` String,
            `port`  UInt32,
            `protocol` String,
            `encoding` String,
        ),
        `metrics`  Tuple(
            `metric_uri`  String,
            `metric_host` String,
            `metric_port` UInt32,
        ),
    )
    ENGINE = MergeTree
    PRIMARY KEY (svc_id)
    ORDER BY (svc_id)
    SETTINGS index_granularity = 128
    "
    )
    .to_string()
}
