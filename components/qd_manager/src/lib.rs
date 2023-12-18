use bar_manager::BarManager;
use common::prelude::DataBar;
use file_manager::FileManager;

#[derive(Debug, Clone, Eq, PartialEq)]
struct QDManager {
    file_manager: FileManager,
    bar_manager: BarManager,
}

impl QDManager {
    fn new() -> Self {
        let file_manager = FileManager::new();

        let bar_manager = BarManager::new();

        Self {
            file_manager,
            bar_manager,
        }
    }
}

impl QDManager {
    fn get_data_bars(&self, symbol: &str) -> Result<Vec<DataBar>, &'static str> {
        match self.bar_manager.get_bars(symbol) {
            Ok(bars) => Ok(bars),
            Err(e) => Err(e),
        }
    }
}
