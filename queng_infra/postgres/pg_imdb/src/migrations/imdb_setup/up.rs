pub(crate) const SCHEMA_UP: &str = r#"
-- Your SQL goes here

CREATE SCHEMA IF NOT EXISTS imdb;

CREATE TYPE imdb.integration_message_config AS (
    id Integer,
    name Text,
    version Integer,
    exchange_id Integer
);

CREATE TABLE imdb.integration_config (
    integration_id VARCHAR(255) NOT NULL PRIMARY KEY,
    integration_version Integer NOT NULL,
    ims_integration_type Integer NOT NULL,
    online BOOLEAN NOT NULL,
    exchange_id Integer NOT NULL,
    integration_message_config imdb.integration_message_config NOT NULL
);

"#;
