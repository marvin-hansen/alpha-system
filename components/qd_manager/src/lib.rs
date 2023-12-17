use bar_manager::BarManager;
use file_manager::FileManager;
use common::prelude::DataBar;

#[derive(Debug, Clone, Eq, PartialEq)]
struct QDManager {
    file_manager: FileManager,
    bar_manager: BarManager,
}


impl QDManager {
    fn new() -> Self {
        let file_manager = FileManager::new();
        let bar_manager = BarManager::new();

        // TODO: load bars from file and add to bar_manager

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
            Err(e) => Err(e)
        }
    }
}
