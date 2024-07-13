fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &[
                "proto/cmdb.proto",
                "proto/dbgw.proto",
                "proto/ims_data.proto",
                "proto/mdm.proto",
                "proto/portfolio_messages.proto",
                "proto/service_messages.proto",
                "proto/smdb.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
