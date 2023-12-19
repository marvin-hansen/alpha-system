use common::prelude::FileConfigType;
use std::str::FromStr;

#[test]
fn test_from_str() {
    assert_eq!(
        FileConfigType::from_str("NoValue"),
        Ok(FileConfigType::NoValue)
    );
    assert_eq!(
        FileConfigType::from_str("EthAll"),
        Ok(FileConfigType::EthAll)
    );
    assert_eq!(
        FileConfigType::from_str("EthSmall"),
        Ok(FileConfigType::EthSmall)
    );
    assert_eq!(
        FileConfigType::from_str("BtcAll"),
        Ok(FileConfigType::BtcAll)
    );
    assert_eq!(
        FileConfigType::from_str("BtcSmall"),
        Ok(FileConfigType::BtcSmall)
    );
    assert_eq!(
        FileConfigType::from_str("BtcMin2017"),
        Ok(FileConfigType::BtcMin2017)
    );
    assert_eq!(
        FileConfigType::from_str("BtcMin2018"),
        Ok(FileConfigType::BtcMin2018)
    );
    assert_eq!(
        FileConfigType::from_str("BtcMin2019"),
        Ok(FileConfigType::BtcMin2019)
    );
    assert_eq!(
        FileConfigType::from_str("BtcMin2020"),
        Ok(FileConfigType::BtcMin2020)
    );
    assert_eq!(
        FileConfigType::from_str("BtcMin2021"),
        Ok(FileConfigType::BtcMin2021)
    );
    assert_eq!(
        FileConfigType::from_str("BtcMin2022"),
        Ok(FileConfigType::BtcMin2022)
    );
    assert_eq!(
        FileConfigType::from_str("BtcMin2023"),
        Ok(FileConfigType::BtcMin2023)
    );
    assert_eq!(FileConfigType::from_str("Invalid"), Err(()));
}

#[test]
fn test_from_u8() {
    assert_eq!(FileConfigType::from(0x0), FileConfigType::NoValue);
    assert_eq!(FileConfigType::from(0x1), FileConfigType::EthAll);
    assert_eq!(FileConfigType::from(0x2), FileConfigType::EthSmall);
    assert_eq!(FileConfigType::from(0x3), FileConfigType::BtcAll);
    assert_eq!(FileConfigType::from(0x4), FileConfigType::BtcSmall);
    assert_eq!(FileConfigType::from(0x5), FileConfigType::BtcMin2017);
    assert_eq!(FileConfigType::from(0x6), FileConfigType::BtcMin2018);
    assert_eq!(FileConfigType::from(0x7), FileConfigType::BtcMin2019);
    assert_eq!(FileConfigType::from(0x8), FileConfigType::BtcMin2020);
    assert_eq!(FileConfigType::from(0x9), FileConfigType::BtcMin2021);
    assert_eq!(FileConfigType::from(0xA), FileConfigType::BtcMin2022);
    assert_eq!(FileConfigType::from(0xB), FileConfigType::BtcMin2023);
}

#[test]
fn test_year() {
    assert_eq!(FileConfigType::NoValue.year(), 0000);
    assert_eq!(FileConfigType::EthAll.year(), 0000);
    assert_eq!(FileConfigType::EthSmall.year(), 0000);
    assert_eq!(FileConfigType::BtcAll.year(), 0000);
    assert_eq!(FileConfigType::BtcSmall.year(), 0000);
    assert_eq!(FileConfigType::BtcMin2017.year(), 2017);
    assert_eq!(FileConfigType::BtcMin2018.year(), 2018);
    assert_eq!(FileConfigType::BtcMin2019.year(), 2019);
    assert_eq!(FileConfigType::BtcMin2020.year(), 2020);
    assert_eq!(FileConfigType::BtcMin2021.year(), 2021);
    assert_eq!(FileConfigType::BtcMin2022.year(), 2022);
    assert_eq!(FileConfigType::BtcMin2023.year(), 2023);
}
