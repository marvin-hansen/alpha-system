/// Creates a new custom reqwest client.
pub(crate) fn get_client() -> reqwest::Client {
    // Enable gzip compressions for requests and responses to reduce download time.
    // https://dtantsur.github.io/rust-openstack/reqwest/struct.ClientBuilder.html
    reqwest::Client::builder()
        .gzip(true)
        .build()
        .expect("Failed to build reqwest client")
}
