use common::prelude::HostEndpoint;

#[test]
fn new() {
    let host_uri = "http://localhost".to_string();
    let port = 8080;
    let host_endpoint = HostEndpoint::new(host_uri, port);

    assert_eq!(host_endpoint.host_uri(), "http://localhost".to_string());
    assert_eq!(host_endpoint.port(), 8080);
}

#[test]
fn host_uri() {
    let host_uri = "http://localhost".to_string();
    let port = 8080;
    let host_endpoint = HostEndpoint::new(host_uri, port);

    assert_eq!(host_endpoint.host_uri(), "http://localhost".to_string());
}

#[test]
fn port() {
    let host_uri = "http://localhost".to_string();
    let port = 8080;
    let host_endpoint = HostEndpoint::new(host_uri.clone(), port);

    assert_eq!(host_endpoint.port(), 8080);
}
