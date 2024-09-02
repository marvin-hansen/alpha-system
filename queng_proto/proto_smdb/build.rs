fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &[
                "proto/dbgw.proto",
                "proto/service_messages.proto",
                "proto/smdb.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
