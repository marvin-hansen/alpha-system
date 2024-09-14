use crate::model::stat::{CreateStat, Stat};
use crate::schema::mddb::stats::dsl::stats as stats_table;
use crate::schema::mddb::stats::stats_id;
use common_metadata::prelude::MetaStats;
use diesel::prelude::*;
use diesel::result::Error::DatabaseError;

impl Stat {
    /// Creates a new stat entry in the database based on the provided MetaStats information.
    ///
    /// # Arguments
    ///
    /// - `conn`: A mutable reference to the database connection.
    /// - `meta_stats`: The MetaStats struct containing the stat information to be inserted.
    ///
    /// # Returns
    ///
    /// A Result containing the inserted MetaStats if successful, or a diesel Error if an error occurs.
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
    /// - `conn`: A mutable reference to the database connection.
    /// - `meta_stats_collection`: A vector of `MetaStats` instances to be inserted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success of the operation, where true represents successful insertion
    /// and an `Error` represents a failure.
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
    /// - `conn`: A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// A Result containing the total count of stat entries if successful, or a diesel Error if an error occurs.
    ///
    pub fn count(conn: &mut crate::Connection) -> Result<u64, diesel::result::Error> {
        stats_table
            .count()
            .get_result::<i64>(conn)
            .map(|count| count as u64)
    }

    pub fn check_if_stat_id_exists(
        conn: &mut crate::Connection,
        param_stat_id: i32,
    ) -> Result<bool, diesel::result::Error> {
        diesel::select(diesel::dsl::exists(
            stats_table.filter(stats_id.eq(param_stat_id)),
        ))
        .get_result(conn)
    }

    /// Reads a stat entry from the database based on the provided stat_id.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `stat_id`: The ID of the stat entry to read.
    ///
    /// # Returns
    /// A Result containing the retrieved MetaStats if successful,
    /// or a diesel Error if an error occurs.
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
    /// - `conn`: A mutable reference to the database connection.
    ///
    /// # Returns
    /// A Result containing a vector of MetaStats if successful, or a diesel Error if an error occurs.
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
    /// Returns a `Result` containing the number of rows affected by the delete operation.
    /// If the statistic does not exist, the query will return `Ok(0)`.
    /// If the statistic exists and was deleted, the query will return `Ok(1)`.
    ///
    /// Note, delete only returns an error when either the database connection or the query fails.
    ///
    pub fn delete(
        conn: &mut crate::Connection,
        stat_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(stats_table.filter(stats_id.eq(stat_id))).execute(conn)
    }
}
