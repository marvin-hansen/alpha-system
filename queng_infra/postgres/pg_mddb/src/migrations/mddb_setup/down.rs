/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

pub const SCHEMA_DOWN: &str = r"
-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS mddb.instruments_exchanges;

DROP TABLE IF EXISTS mddb.assets;
DROP TABLE IF EXISTS mddb.exchanges;
DROP TABLE IF EXISTS mddb.stats;

DROP schema IF EXISTS mddb;
";
