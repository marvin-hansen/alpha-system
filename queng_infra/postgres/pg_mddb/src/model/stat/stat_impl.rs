use crate::model::stat::{CreateStat, Stat};
use crate::schema::mddb::stats::dsl::stats as stats_table;
use crate::schema::mddb::stats::stats_id;
use common_metadata::MetaStats;
use diesel::prelude::*;
use diesel::result::Error::DatabaseError;

impl Stat {
    /// Creates a new stat entry in the database based on the provided `MetaStats` information.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_stats` - The `MetaStats` struct containing the stat information to be inserted.
    ///
    /// # Returns
    ///
    /// Returns a `Result<MetaStats, diesel::result::Error>`:
    /// * `Ok(meta_stats)` - The newly created stat entry, converted back to `MetaStats`
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Unique constraint violations (e.g., duplicate `stat_id`)
    /// * Invalid data in `meta_stats` that violates database constraints
    /// * Transaction failure during the insert operation
    /// * Data conversion errors between `MetaStats` and database types
    ///
    pub fn create(
        conn: &mut crate::Connection,
        meta_stats: MetaStats,
    ) -> Result<MetaStats, diesel::result::Error> {
        let new_stat = CreateStat::from_meta_stats(meta_stats);
        diesel::insert_into(stats_table)
            .values(&new_stat)
            .get_result::<Self>(conn)
            .map(|stat| stat.to_meta_stats())
    }

    /// Inserts a collection of statistics into the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_stats_collection` - A vector of `MetaStats` instances to be inserted.
    ///
    /// # Returns
    ///
    /// Returns a `Result<bool, diesel::result::Error>`:
    /// * `Ok(true)` - All stats were successfully inserted
    /// * `Err(_)` - The operation failed (no partial insertions - it's all or nothing)
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Empty collection provided (returns a `DatabaseError` with Unknown kind)
    /// * Database connection error
    /// * Unique constraint violations in any of the stats
    /// * Invalid data in any `meta_stats` that violates database constraints
    /// * Transaction failure during the bulk insert operation
    /// * Data conversion errors between `MetaStats` and database types
    ///
    pub fn create_stat_collection(
        conn: &mut crate::Connection,
        meta_stats_collection: Vec<MetaStats>,
    ) -> Result<bool, diesel::result::Error> {
        if meta_stats_collection.is_empty() {
            return Err(DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(String::from(
                    "[create_stat_collection]: No stats provided. Collection is empty. ",
                )),
            ));
        }

        let new_stats: Vec<CreateStat> = meta_stats_collection
            .into_iter()
            .map(CreateStat::from_meta_stats)
            .collect();
        diesel::insert_into(stats_table)
            .values(&new_stats)
            .execute(conn)
            .map(|_| true)
    }

    /// Counts the number of stat entries in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// Returns a `Result<u64, diesel::result::Error>`:
    /// * `Ok(count)` - The total number of stat entries in the database
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Integer overflow when converting count from i64 to u64 (extremely unlikely)
    ///
    pub fn count(conn: &mut crate::Connection) -> Result<u64, diesel::result::Error> {
        stats_table
            .count()
            .get_result::<i64>(conn)
            .map(|count| count as u64)
    }

    /// Checks if a stat entry exists in the database with the given ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `param_stat_id` - The ID of the stat to check for existence.
    ///
    /// # Returns
    ///
    /// Returns a `Result<bool, diesel::result::Error>`:
    /// * `Ok(true)` - A stat with the given ID exists
    /// * `Ok(false)` - No stat exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Type conversion errors when processing the result
    ///
    pub fn check_if_stat_id_exists(
        conn: &mut crate::Connection,
        param_stat_id: i32,
    ) -> Result<bool, diesel::result::Error> {
        diesel::select(diesel::dsl::exists(
            stats_table.filter(stats_id.eq(param_stat_id)),
        ))
        .get_result(conn)
    }

    /// Reads a stat entry from the database based on the provided `stat_id`.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `stat_id` - The ID of the stat entry to read.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<MetaStats>, diesel::result::Error>`:
    /// * `Ok(Some(meta_stats))` - The stat was found and successfully retrieved
    /// * `Ok(None)` - No stat exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization error when converting database record to `MetaStats`
    /// * Type conversion errors when processing the result
    ///
    pub fn read(
        conn: &mut crate::Connection,
        stat_id: i32,
    ) -> Result<MetaStats, diesel::result::Error> {
        stats_table
            .filter(stats_id.eq(stat_id))
            .first::<Self>(conn)
            .map(|stat| stat.to_meta_stats())
    }

    /// Reads all stat entries from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Vec<MetaStats>, diesel::result::Error>`:
    /// * `Ok(vec)` - A vector containing all stats in the database
    /// * Returns an empty vector if no stats exist
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization errors when converting database records to `MetaStats`
    /// * Memory allocation errors when dealing with large result sets
    ///
    pub fn read_all(conn: &mut crate::Connection) -> Result<Vec<MetaStats>, diesel::result::Error> {
        stats_table
            .load::<Self>(conn)
            .map(|stats| stats.into_iter().map(|stat| stat.to_meta_stats()).collect())
    }

    /// Deletes a statistic from the database by statistic ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `stat_id` - The ID of the statistic to delete.
    ///
    /// # Returns
    ///
    /// Returns a `Result<usize, diesel::result::Error>`:
    /// * `Ok(1)` - The stat was successfully deleted
    /// * `Ok(0)` - No stat exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Foreign key constraint violations if the stat is referenced by other tables
    /// * Transaction failure during the delete operation
    /// * Concurrent modification conflicts
    ///
    pub fn delete(
        conn: &mut crate::Connection,
        stat_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(stats_table.filter(stats_id.eq(stat_id))).execute(conn)
    }
}
