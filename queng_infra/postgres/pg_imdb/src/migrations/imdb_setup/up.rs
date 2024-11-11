pub(crate) const SCHEMA_UP: &str = r#"
-- Your SQL goes here

CREATE SCHEMA IF NOT EXISTS imdb;

CREATE TYPE imdb.integration_message_config AS (
    id SMALLINT,
    name VARCHAR(255),
    version SMALLINT,
    exchange_id SMALLINT
);

CREATE TABLE imdb.integration_config (
    integration_id VARCHAR(255) NOT NULL PRIMARY KEY,
    integration_version SMALLINT NOT NULL,
    ims_integration_type SMALLINT NOT NULL,
    online BOOLEAN NOT NULL,
    exchange_id SMALLINT NOT NULL,
    integration_message_config imdb.integration_message_config NOT NULL
);

"#;
