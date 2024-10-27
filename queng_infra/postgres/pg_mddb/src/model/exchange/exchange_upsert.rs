use crate::prelude::Exchange;
use crate::Connection;
use common_metadata::prelude::MetaExchange;
use diesel::result::Error;
use diesel::result::Error::DatabaseError;
use diesel::RunQueryDsl;

impl Exchange {
    //
    pub fn merge_upsert_collection(
        conn: &mut Connection,
        meta_exchanges: &[MetaExchange],
    ) -> Result<usize, Error> {
        // Check if empty
        if meta_exchanges.is_empty() {
            return Err(DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(String::from(
                    "[create_exchange_collection] No exchanges provided. Collection is empty.",
                )),
            ));
        }

        // A temporary table called tmp_exchanges_new as raw string
        // https://neon.tech/postgresql/postgresql-tutorial/postgresql-temporary-table
        let create_ddl: &str = r#"
        "CREATE TEMPORARY TABLE tmp_exchanges_new(
        	"exchange_id" VARCHAR NOT NULL PRIMARY KEY,
            "exchange_hash" VARCHAR NOT NULL,
            "exchange_name" VARCHAR NOT NULL
        )"; "#;

        // 1. Create a temporary table called tmp_exchanges_new using a raw sql query with diesel
        diesel::sql_query(create_ddl)
            .execute(conn)
            .expect("Failed to create temporary exchange table");

        // 2. Insert the meta_exchanges collection into the temporary table
        // https://neon.tech/postgresql/postgresql-tutorial/postgresql-insert-multiple-rows
        let insert_query: &str = r#"
        "INSERT INTO tmp_exchanges_new(exchange_id,exchange_hash, exchange_name)"
        "VALUES (?1,?2,?3)";
         "#;

        // Res returns the number of rows inserted
        let number_inserted = diesel::sql_query(insert_query)
            .execute(conn)
            .expect("Failed to drop temporary exchange table");

        // 3. Check if the main table contains any rows not present in the temporary table.

        //    If yes, delete them from the main table.
        //    If no, proceed and merge upsert from the temporary table into the main table using merge in a raw sql query

        // 4. Check if the count of the main table matches the count of the temporary table

        // 5. Drop the temporary table
        let drop_ddl: &str = r#"DROP TABLE tmp_exchanges_new;"#;
        diesel::sql_query(drop_ddl)
            .execute(conn)
            .expect("Failed to drop temporary exchange table");

        Ok(0)
    }
}
