use common_message::{ImsDataConfig, StreamUser};

#[test]
fn test_ims_data_config_new() {
    let stream_user = StreamUser::new("test_user", "test_pass");
    let config = ImsDataConfig::new(
        stream_user.clone(),
        "stream1".to_string(),
        "topic1,topic2".to_string(),
        "127.0.0.1:8090".to_string(),
    );

    assert_eq!(config.stream_user(), &stream_user);
    assert_eq!(config.stream_id(), "stream1");
    assert_eq!(config.topic_ids(), "topic1,topic2");
    assert_eq!(config.tcp_server_address(), "127.0.0.1:8090");
}

#[test]
fn test_ims_data_config_default() {
    let config = ImsDataConfig::default();
    assert_eq!(config.stream_user(), &StreamUser::default());
    assert_eq!(config.stream_id(), "");
    assert_eq!(config.topic_ids(), "");
    assert_eq!(config.tcp_server_address(), "");
}

#[test]
fn test_ims_data_config_display() {
    let stream_user = StreamUser::new("test_user", "test_pass");
    let config = ImsDataConfig::new(
        stream_user,
        "stream1".to_string(),
        "topic1".to_string(),
        "127.0.0.1:8090".to_string(),
    );
    let display_str = format!("{}", config);
    assert!(display_str.contains("test_user"));
    assert!(display_str.contains("stream1"));
    assert!(display_str.contains("topic1"));
    assert!(display_str.contains("127.0.0.1:8090"));
}

#[test]
fn test_ims_data_config_clone() {
    let stream_user = StreamUser::new("test_user", "test_pass");
    let config = ImsDataConfig::new(
        stream_user,
        "stream1".to_string(),
        "topic1".to_string(),
        "127.0.0.1:8090".to_string(),
    );
    let cloned = config.clone();
    assert_eq!(config, cloned);
}
