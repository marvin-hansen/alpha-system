use common_ims::ExchangeDataIntegrationID;

#[test]
fn test_enum_values() {
    assert_eq!(ExchangeDataIntegrationID::BinanceData as u8, 0);
    assert_eq!(ExchangeDataIntegrationID::MockData as u8, 1);
    assert_eq!(ExchangeDataIntegrationID::VexData as u8, 2);
}

#[test]
fn test_display_trait() {
    assert_eq!(
        format!("{}", ExchangeDataIntegrationID::BinanceData),
        "BinanceData"
    );
    assert_eq!(
        format!("{}", ExchangeDataIntegrationID::MockData),
        "MockData"
    );
    assert_eq!(format!("{}", ExchangeDataIntegrationID::VexData), "VexData");
}

#[test]
fn test_partial_eq_trait() {
    assert_eq!(
        ExchangeDataIntegrationID::BinanceData,
        ExchangeDataIntegrationID::BinanceData
    );
    assert_ne!(
        ExchangeDataIntegrationID::BinanceData,
        ExchangeDataIntegrationID::MockData
    );
}

#[test]
fn test_partial_ord_trait() {
    assert!(ExchangeDataIntegrationID::BinanceData <= ExchangeDataIntegrationID::MockData);
    assert!(ExchangeDataIntegrationID::MockData >= ExchangeDataIntegrationID::BinanceData);
}

#[test]
fn test_ord_trait() {
    assert!(ExchangeDataIntegrationID::BinanceData < ExchangeDataIntegrationID::MockData);
    assert!(ExchangeDataIntegrationID::MockData > ExchangeDataIntegrationID::BinanceData);
}

#[test]
fn test_clone_trait() {
    let original = ExchangeDataIntegrationID::BinanceData;
    let cloned = original;
    assert_eq!(original, cloned);
}

#[test]
fn test_copy_trait() {
    let original = ExchangeDataIntegrationID::BinanceData;
    let copied = original;
    assert_eq!(original, copied);
}

#[test]
fn test_debug_trait() {
    assert_eq!(
        format!("{:?}", ExchangeDataIntegrationID::BinanceData),
        "BinanceData"
    );
    assert_eq!(
        format!("{:?}", ExchangeDataIntegrationID::MockData),
        "MockData"
    );
    assert_eq!(
        format!("{:?}", ExchangeDataIntegrationID::VexData),
        "VexData"
    );
}
