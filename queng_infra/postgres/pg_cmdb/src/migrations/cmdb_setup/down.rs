pub const SCHEMA_DOWN: &'static str = r#"
-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS cmdb.portfolio_instrument;
DROP TABLE IF EXISTS cmdb.portfolio;
DROP TABLE IF EXISTS cmdb.instrument;
DROP schema IF EXISTS cmdb;
"#;
