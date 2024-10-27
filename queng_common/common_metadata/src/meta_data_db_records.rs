use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MetaDataDBRecords {
    number_db_assets: u32,
    number_db_exchanges: u32,
    number_db_instruments: u32,
}

impl MetaDataDBRecords {
    pub fn new(
        number_db_assets: u32,
        number_db_exchanges: u32,
        number_db_instruments: u32,
    ) -> Self {
        Self {
            number_db_assets,
            number_db_exchanges,
            number_db_instruments,
        }
    }
}

impl MetaDataDBRecords {
    pub fn number_db_assets(&self) -> u32 {
        self.number_db_assets
    }

    pub fn number_db_exchanges(&self) -> u32 {
        self.number_db_exchanges
    }

    pub fn number_db_instruments(&self) -> u32 {
        self.number_db_instruments
    }
}

impl Display for MetaDataDBRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumberDataBaseRecords {{ number_db_assets: {}, number_db_exchanges: {}, number_db_instruments: {} }}", self.number_db_assets, self.number_db_exchanges, self.number_db_instruments)
    }
}
