fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &[
                "proto/mddb.proto",
                "proto/mddb_assets_messages.proto",
                "proto/mddb_exchanges_messages.proto",
                "proto/mddb_instruments_messages.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
