-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS smdb.service;
DROP TYPE IF EXISTS smdb.service_endpoint CASCADE;
DROP schema IF EXISTS smdb;