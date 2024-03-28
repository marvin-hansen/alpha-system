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
        `base_uri` String CODEC(LZ4),
        `dependencies` Array(Nullable(String)) CODEC(LZ4),
        `exposure` String CODEC(LZ4),
        `endpoint`  Tuple(
            description CODEC(LZ4),
            encoding CODEC(LZ4),
            name Nullable(String),
            port UInt32 CODEC(Delta, LZ4),
            protocol String CODEC(LZ4),
            uri String CODEC(LZ4),
            version UInt8 CODEC(LZ4)
        ) CODEC(LZ4),
        `metrics`  Tuple(
            metric_host CODEC(LZ4),
            metric_port UInt32 CODEC(Delta, LZ4),
            metric_uri CODEC(LZ4)
        ) CODEC(LZ4),
    )
    ENGINE = MergeTree
    PRIMARY KEY (svc_id)
    ORDER BY (svc_id)
    SETTINGS index_granularity = 1024
    "
    )
    .to_string()
}
