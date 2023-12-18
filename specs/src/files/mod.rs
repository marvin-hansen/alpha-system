use std::collections::HashMap;
use common::prelude::{FileConfig, FileConfigType};

pub mod btc_1m_2017;
pub mod btc_1m_2018;
pub mod btc_1m_2019;
pub mod btc_1m_2020;
pub mod btc_1m_2021;
pub mod btc_1m_2022;
pub mod btc_small;

pub fn get_all_file_configs() -> HashMap<FileConfigType, FileConfig> {
    let mut file_configs = HashMap::new();
    file_configs.insert(FileConfigType::BtcMin2017, btc_1m_2017::get_btc_usd_2017_file_config());
    file_configs.insert(FileConfigType::BtcMin2018, btc_1m_2018::get_btc_usd_2018_file_config());
    file_configs.insert(FileConfigType::BtcMin2019, btc_1m_2019::get_btc_usd_2019_file_config());
    file_configs.insert(FileConfigType::BtcMin2020, btc_1m_2020::get_btc_usd_2020_file_config());
    file_configs.insert(FileConfigType::BtcMin2021, btc_1m_2021::get_btc_usd_2021_file_config());
    file_configs.insert(FileConfigType::BtcMin2022, btc_1m_2022::get_btc_usd_2022_file_config());

    file_configs
}