use clickhouse_utils::prelude::DataImportConfig;

pub(crate) fn meta_data_import_config() -> DataImportConfig<'static> {
    DataImportConfig::new("", "", "")
}
