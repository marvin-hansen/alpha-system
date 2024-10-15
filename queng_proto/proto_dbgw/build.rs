fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &[
                "proto/dbgw.proto",
                "portfolio_messages.proto",
                "service_messages.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
