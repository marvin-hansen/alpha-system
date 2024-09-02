fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/mddb.proto", "proto/mddb_messages.proto"],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
