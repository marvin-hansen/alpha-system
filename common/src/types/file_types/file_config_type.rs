use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum FileConfigType {
    #[default]
    NoValue = 0x0_u8,
    EthAll = 0x1_u8,
    EthSmall = 0x2_u8,
    BtcAll = 0x3_u8,
    BtcSmall = 0x4_u8,
    BtcMin2017 = 0x5_u8,
    BtcMin2018 = 0x6_u8,
    BtcMin2019 = 0x7_u8,
    BtcMin2020 = 0x8_u8,
    BtcMin2021 = 0x9_u8,
    BtcMin2022 = 0xA_u8,
    BtcMin2023 = 0xB_u8,
}

impl FromStr for FileConfigType {
    type Err = ();

    // This allows creating a FileConfigType from a &str
    // by matching the string value to the variant names.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoValue" => Ok(FileConfigType::NoValue),
            "EthAll" => Ok(FileConfigType::EthAll),
            "EthSmall" => Ok(FileConfigType::EthSmall),
            "BtcAll" => Ok(FileConfigType::BtcAll),
            "BtcSmall" => Ok(FileConfigType::BtcSmall),
            "BtcMin2017" => Ok(FileConfigType::BtcMin2017),
            "BtcMin2018" => Ok(FileConfigType::BtcMin2018),
            "BtcMin2019" => Ok(FileConfigType::BtcMin2019),
            "BtcMin2020" => Ok(FileConfigType::BtcMin2020),
            "BtcMin2021" => Ok(FileConfigType::BtcMin2021),
            "BtcMin2022" => Ok(FileConfigType::BtcMin2022),
            "BtcMin2023" => Ok(FileConfigType::BtcMin2023),
            _ => Err(()),
        }
    }
}

// This implements the conversion from a u8 value to a FileConfigType
// variant by matching on the numeric value.
impl From<u8> for FileConfigType {
    fn from(value: u8) -> Self {
        match value {
            0x0 => FileConfigType::NoValue,
            0x1 => FileConfigType::EthAll,
            0x2 => FileConfigType::EthSmall,
            0x3 => FileConfigType::BtcAll,
            0x4 => FileConfigType::BtcSmall,
            0x5 => FileConfigType::BtcMin2017,
            0x6 => FileConfigType::BtcMin2018,
            0x7 => FileConfigType::BtcMin2019,
            0x8 => FileConfigType::BtcMin2020,
            0x9 => FileConfigType::BtcMin2021,
            0xA => FileConfigType::BtcMin2022,
            0xB => FileConfigType::BtcMin2023,
            _ => FileConfigType::NoValue,
        }
    }
}

impl FileConfigType {
    pub fn year(&self) -> u16 {
        match self {
            FileConfigType::NoValue => 0000,
            FileConfigType::EthAll => 0000,
            FileConfigType::EthSmall => 0000,
            FileConfigType::BtcAll => 0000,
            FileConfigType::BtcSmall => 0000,
            FileConfigType::BtcMin2017 => 2017,
            FileConfigType::BtcMin2018 => 2018,
            FileConfigType::BtcMin2019 => 2019,
            FileConfigType::BtcMin2020 => 2020,
            FileConfigType::BtcMin2021 => 2021,
            FileConfigType::BtcMin2022 => 2022,
            FileConfigType::BtcMin2023 => 2023,
        }
    }
}

impl Display for FileConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
