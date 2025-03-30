/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

//! # Schema Module
//!
//! This module contains the schema definitions for the `clickhouse_utils` crate.
//!
//! The schema definitions are used to describe the structure and relationships of the
//! tables in the `metadata` database in clickhouse.
//!
//! The module contains the following submodules:
//!
//! - `stats_table_schema`: defines the schema for the `stats_table`
//! - `assets_table_schema`: defines the schema for the `assets_table`
//! - `exchanges_table_schema`: defines the schema for the `exchanges_table`
//! - `instruments_table_schema`: defines the schema for the `instruments_table`
//!
//! Each submodule contains the necessary code to define the schema for the corresponding table.
//!
mod assets_table_schema;
mod exchanges_table_schema;
mod instruments_table_schema;
mod stats_table_schema;
