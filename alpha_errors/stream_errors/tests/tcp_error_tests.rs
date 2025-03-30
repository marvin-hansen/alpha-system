use stream_errors::TcpError;

#[test]
fn test_tcp_error_display() {
    assert_eq!(TcpError::UnknownTcpError.to_string(), "unknown TCP error");
    assert_eq!(
        TcpError::FailedToSetTcpNodelay.to_string(),
        "failed to set TCP NO_DELAY"
    );
    assert_eq!(
        TcpError::FailedToSetSoKeepAlive.to_string(),
        "failed to set SO_KEEPALIVE"
    );
    assert_eq!(
        TcpError::FailedToSetSoReuseaddr.to_string(),
        "failed to set SO_REUSEADDR"
    );
    assert_eq!(
        TcpError::FailedToSetSoRcvbuf.to_string(),
        "failed to set SO_RCVBUF"
    );
    assert_eq!(
        TcpError::FailedToSetSoSndbuf.to_string(),
        "failed to set SO_SNDBUF"
    );
    assert_eq!(
        TcpError::FailedToSetSoReuseport.to_string(),
        "failed to set SO_REUSEPORT"
    );
}

#[test]
fn test_tcp_error_into_u8() {
    assert_eq!(TcpError::UnknownTcpError as u8, 0);
    assert_eq!(TcpError::FailedToSetTcpNodelay as u8, 1);
    assert_eq!(TcpError::FailedToSetSoKeepAlive as u8, 2);
    assert_eq!(TcpError::FailedToSetSoReuseaddr as u8, 6);
    assert_eq!(TcpError::FailedToSetSoRcvbuf as u8, 4);
    assert_eq!(TcpError::FailedToSetSoSndbuf as u8, 5);
    assert_eq!(TcpError::FailedToSetSoReuseport as u8, 7);
}
