use crate::prelude::{SymbolID, TimeResolution};
use crate::types::file_types::file_config_type::FileConfigType;

use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FileConfig {
    data_symbol: SymbolID,
    time_resolution: TimeResolution,
    path: String,
    description: String,
    file_config_type: FileConfigType,
}

impl FileConfig {
    pub fn new(
        data_symbol: SymbolID,
        time_resolution: TimeResolution,
        path: String,
        description: String,
        file_config_type: FileConfigType,
    ) -> Self {
        Self {
            data_symbol,
            time_resolution,
            path,
            description,
            file_config_type,
        }
    }
}

impl FileConfig {
    pub fn data_symbol(&self) -> SymbolID {
        self.data_symbol
    }
    pub fn time_resolution(&self) -> TimeResolution {
        self.time_resolution
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn file_config_type(&self) -> &FileConfigType {
        &self.file_config_type
    }
}

impl fmt::Display for FileConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FileConfig[symbol={}, resolution={}, path={}, description={}, type={}]",
            self.data_symbol,
            self.time_resolution,
            self.path,
            self.description,
            self.file_config_type
        )
    }
}
