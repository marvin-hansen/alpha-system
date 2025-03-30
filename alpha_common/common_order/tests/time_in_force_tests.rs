/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::TimeInForce;

#[test]
fn test_time_in_force_conversion_u8() {
    assert_eq!(u8::from(TimeInForce::GoodTillCancel), 0x1_u8);
    assert_eq!(u8::from(TimeInForce::GoodTillTimeExchange), 0x2_u8);
    assert_eq!(u8::from(TimeInForce::FillOrKill), 0x3_u8);
    assert_eq!(u8::from(TimeInForce::ImmediateOrCancel), 0x4_u8);
    assert_eq!(u8::from(TimeInForce::OneCancelOther), 0x5_u8);

    assert_eq!(TimeInForce::from(0x1_u8), TimeInForce::GoodTillCancel);
    assert_eq!(TimeInForce::from(0x2_u8), TimeInForce::GoodTillTimeExchange);
    assert_eq!(TimeInForce::from(0x3_u8), TimeInForce::FillOrKill);
    assert_eq!(TimeInForce::from(0x4_u8), TimeInForce::ImmediateOrCancel);
    assert_eq!(TimeInForce::from(0x5_u8), TimeInForce::OneCancelOther);
    // Test default fallback
    assert_eq!(TimeInForce::from(0x6_u8), TimeInForce::GoodTillCancel);
}

#[test]
fn test_time_in_force_conversion_u16() {
    assert_eq!(u16::from(TimeInForce::GoodTillCancel), 0x1_u16);
    assert_eq!(u16::from(TimeInForce::GoodTillTimeExchange), 0x2_u16);
    assert_eq!(u16::from(TimeInForce::FillOrKill), 0x3_u16);
    assert_eq!(u16::from(TimeInForce::ImmediateOrCancel), 0x4_u16);
    assert_eq!(u16::from(TimeInForce::OneCancelOther), 0x5_u16);

    assert_eq!(TimeInForce::from(0x1_u16), TimeInForce::GoodTillCancel);
    assert_eq!(
        TimeInForce::from(0x2_u16),
        TimeInForce::GoodTillTimeExchange
    );
    assert_eq!(TimeInForce::from(0x3_u16), TimeInForce::FillOrKill);
    assert_eq!(TimeInForce::from(0x4_u16), TimeInForce::ImmediateOrCancel);
    assert_eq!(TimeInForce::from(0x5_u16), TimeInForce::OneCancelOther);
    // Test default fallback
    assert_eq!(TimeInForce::from(0x6_u16), TimeInForce::GoodTillCancel);
}

#[test]
fn test_time_in_force_conversion_u32() {
    assert_eq!(u32::from(TimeInForce::GoodTillCancel), 0x1_u32);
    assert_eq!(u32::from(TimeInForce::GoodTillTimeExchange), 0x2_u32);
    assert_eq!(u32::from(TimeInForce::FillOrKill), 0x3_u32);
    assert_eq!(u32::from(TimeInForce::ImmediateOrCancel), 0x4_u32);
    assert_eq!(u32::from(TimeInForce::OneCancelOther), 0x5_u32);
}

#[test]
fn test_time_in_force_display() {
    assert_eq!(TimeInForce::GoodTillCancel.to_string(), "GoodTillCancel");
    assert_eq!(
        TimeInForce::GoodTillTimeExchange.to_string(),
        "GoodTillTimeExchange"
    );
    assert_eq!(TimeInForce::FillOrKill.to_string(), "FillOrKill");
    assert_eq!(
        TimeInForce::ImmediateOrCancel.to_string(),
        "ImmediateOrCancel"
    );
    assert_eq!(TimeInForce::OneCancelOther.to_string(), "OneCancelOther");
}

#[test]
fn test_time_in_force_debug() {
    assert_eq!(
        format!("{:?}", TimeInForce::GoodTillCancel),
        "GoodTillCancel"
    );
    assert_eq!(
        format!("{:?}", TimeInForce::GoodTillTimeExchange),
        "GoodTillTimeExchange"
    );
    assert_eq!(format!("{:?}", TimeInForce::FillOrKill), "FillOrKill");
    assert_eq!(
        format!("{:?}", TimeInForce::ImmediateOrCancel),
        "ImmediateOrCancel"
    );
    assert_eq!(
        format!("{:?}", TimeInForce::OneCancelOther),
        "OneCancelOther"
    );
}

#[test]
fn test_time_in_force_default() {
    assert_eq!(TimeInForce::default(), TimeInForce::GoodTillCancel);
}

#[test]
fn test_time_in_force_clone_and_eq() {
    let gtc = TimeInForce::GoodTillCancel;
    let gtc_clone = gtc.clone();
    assert_eq!(gtc, gtc_clone);

    let gte = TimeInForce::GoodTillTimeExchange;
    assert_ne!(gtc, gte);

    let fok = TimeInForce::FillOrKill;
    assert_ne!(gtc, fok);
    assert_ne!(gte, fok);

    let ioc = TimeInForce::ImmediateOrCancel;
    assert_ne!(gtc, ioc);

    let oco = TimeInForce::OneCancelOther;
    assert_ne!(gtc, oco);
}
