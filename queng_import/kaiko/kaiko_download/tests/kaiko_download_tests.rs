use kaiko_download::{download_meta_data, download_meta_data_stats};

#[tokio::test]
async fn test_download_meta_data_stats() {
    // Set ENV to CI
    unsafe { std::env::set_var("ENV", "CI") };

    // Test with auto_detect_proxy = true
    let result = download_meta_data_stats(true, true).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_download_meta_data() {
    // Set ENV to CI
    unsafe { std::env::set_var("ENV", "CI") };

    // Test with auto_detect_proxy = true
    let result = download_meta_data(true, true).await;
    assert!(result.is_ok());

    let metedata = result.unwrap();

    let assets = metedata.assets().data.to_vec();
    assert!(assets.len() > 0);

    let instruments = metedata.instruments().data.to_vec();
    assert!(instruments.len() > 0);

    let exchanges = metedata.exchanges().data.to_vec();
    assert!(exchanges.len() > 0);
}
