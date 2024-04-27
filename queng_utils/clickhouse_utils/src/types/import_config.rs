pub struct DataImportConfig<'l> {
    assets_data_path: &'l str,
    exchanges_data_path: &'l str,
    instruments_data_path: &'l str,
}

impl<'l> DataImportConfig<'l> {
    pub fn new(
        assets_data_path: &'l str,
        exchanges_data_path: &'l str,
        instruments_data_path: &'l str,
    ) -> Self {
        Self {
            assets_data_path,
            exchanges_data_path,
            instruments_data_path,
        }
    }
}

impl<'l> DataImportConfig<'l> {
    pub fn assets_data_path(&self) -> &'l str {
        self.assets_data_path
    }
    pub fn exchanges_data_path(&self) -> &'l str {
        self.exchanges_data_path
    }
    pub fn instruments_data_path(&self) -> &'l str {
        self.instruments_data_path
    }
}
