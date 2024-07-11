fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &[
                "proto/cmdb.proto",
                "proto/data_messages.proto",
                "proto/dbgw.proto",
                "proto/exchange_symbol_messages.proto",
                "proto/ims_data.proto",
                "proto/portfolio_messages.proto",
                "proto/service_messages.proto",
                "proto/smdb.proto",
                "proto/symdb.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto specification");

    Ok(())
}
