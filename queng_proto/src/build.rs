fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/cmdb.proto", "proto/portfolio_messages.proto"],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
