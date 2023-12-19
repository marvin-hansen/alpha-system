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

        for file_config in cfg_manager.get_all_file_config_types().iter() {
            let config = cfg_manager
                .get_file_config(file_config)
                .expect("QDManager: Error reading config file");

            let bars = file_manager
                .read_data_from_file(config)
                .expect("QDManager: Error reading data from file");

            bar_manager.add_bars(file_config, bars);
        }

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
