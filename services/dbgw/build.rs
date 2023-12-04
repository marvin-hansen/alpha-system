
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/dbgw.proto")
        .expect("Failed to compile proto specification");
    Ok(())
}
