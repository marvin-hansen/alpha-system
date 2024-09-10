// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
pub mod proto {
    tonic::include_proto!("proto");
}
