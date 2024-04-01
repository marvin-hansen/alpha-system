use client_utils::config_utils::ConfigFile;

pub(crate) fn get_meta_data_config() -> ConfigFile {
    ConfigFile::new("data/kaiko")
}
