fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/imdb_messages.proto", "proto/imdb.proto"],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
