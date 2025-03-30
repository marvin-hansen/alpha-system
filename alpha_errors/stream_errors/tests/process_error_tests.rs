use stream_errors::MessageProcessorError;

#[test]
fn test_display() {
    assert_eq!(
        format!("{}", MessageProcessorError::new("error".to_string())),
        "error"
    );
    assert_eq!(
        format!("{}", MessageProcessorError("error".to_string())),
        "error"
    );
}

#[test]
fn test_clone() {
    let error = MessageProcessorError("error".to_string());
    assert_eq!(error.clone(), error);
}

#[test]
fn test_partial_eq() {
    let error = MessageProcessorError("error".to_string());
    assert_eq!(error, error.clone());
    assert_eq!(error, MessageProcessorError("error".to_string()));
}

#[test]
fn test_debug() {
    let error = MessageProcessorError("error".to_string());
    assert_eq!(format!("{:?}", error), "MessageProcessorError(\"error\")");
}
