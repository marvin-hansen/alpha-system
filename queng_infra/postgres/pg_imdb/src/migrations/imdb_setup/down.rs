pub const SCHEMA_DOWN: &str = r"
-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS imdb.integration_config;
DROP TYPE IF EXISTS imdb.integration_message_config;
DROP schema IF EXISTS imdb;
";
