fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &[
                "proto/cmdb.proto",
                "proto/dbgw.proto",
                "proto/ims_data.proto",
                "proto/smdb.proto",
                "proto/symdb.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
