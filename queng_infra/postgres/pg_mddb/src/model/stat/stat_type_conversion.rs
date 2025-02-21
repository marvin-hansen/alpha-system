/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::CreateStat;
use crate::model::stat::Stat;
use common_metadata::MetaStats;

impl Stat {
    /// Converts `Stats` to `PostgresStat`
    ///
    /// This method takes `Stats` and returns `PostgresStat` for saving to the database.
    ///
    /// It is used in the following workflows:
    ///
    /// - When downloading meta data set from Kaiko the `Stats` is converted from the downloaded JSON
    ///   to `PostgresStat` and then saved to the database.
    /// - When retrieving meta data set from database the `PostgresStat` is converted to `Stats`
    ///   and returned to the user.
    ///
    #[must_use]
    pub fn from_meta_stats(meta_stats: MetaStats) -> Self {
        Self {
            stats_id: 0, // Assuming stats_id is auto-generated
            stats_hash: meta_stats.hash().to_string(),
            stats_download_timestamp: meta_stats.download_timestamp().to_string(),
            stats_number_assets: meta_stats.number_assets() as i32,
            stats_number_exchanges: meta_stats.number_exchanges() as i32,
            stats_number_instruments: meta_stats.number_instruments() as i32,
        }
    }
    /// Converts `PostgresStat` to `Stats`
    ///
    /// This method takes `PostgresStat` and returns `Stats` from `common_metadata` crate.
    ///
    /// It is used in the following workflows:
    ///
    /// - When downloading meta data set from Kaiko the `Stats` is converted from the downloaded JSON
    ///   to `PostgresStat` and then saved to the database.
    /// - When retrieving meta data set from database the `PostgresStat` is converted to `Stats`
    ///   and returned to the user.
    ///
    #[must_use]
    pub fn to_meta_stats(&self) -> MetaStats {
        MetaStats::new(
            self.stats_download_timestamp.clone(),
            self.stats_hash.clone(),
            self.stats_number_assets as u32,
            self.stats_number_exchanges as u32,
            self.stats_number_instruments as u32,
        )
    }
}

impl CreateStat {
    #[must_use]
    pub fn from_meta_stats(meta_stats: MetaStats) -> Self {
        Self {
            stats_id: 0, // Assuming stats_id is auto-generated
            stats_hash: meta_stats.hash().to_string(),
            stats_download_timestamp: meta_stats.download_timestamp().to_string(),
            stats_number_assets: meta_stats.number_assets() as i32,
            stats_number_exchanges: meta_stats.number_exchanges() as i32,
            stats_number_instruments: meta_stats.number_instruments() as i32,
        }
    }
}
