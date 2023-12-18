use bar_manager::BarManager;
use cfg_manager::CfgManager;
use common::prelude::{DataBar, FileConfigType};
use file_manager::FileManager;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QDManager {
    file_manager: FileManager,
    bar_manager: BarManager,
}

impl QDManager {
    pub fn new(cfg_manager: &CfgManager) -> Self {

        let file_manager = FileManager::new();

        let mut bar_manager = BarManager::new();

        let file_config_type = &FileConfigType::BtcSmall;

        let btc_small_config = cfg_manager
            .get_file_config(file_config_type)
            .expect("QDManager: Error reading config file");

        let btc_bars = file_manager
            .read_data_from_file(btc_small_config)
            .expect("QDManager: Error reading data from file");

        bar_manager.add_bars(file_config_type, btc_bars);

        Self {
            file_manager,
            bar_manager,
        }
    }
}

impl QDManager {
    pub fn get_data_bars(&self, symbol: &FileConfigType) -> Result<Vec<DataBar>, &'static str> {
        match self.bar_manager.get_bars(symbol) {
            Ok(bars) => Ok(bars),
            Err(e) => Err(e),
        }
    }
}
