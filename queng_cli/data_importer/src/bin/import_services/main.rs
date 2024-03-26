mod gen_ddl;
mod gen_query;
mod process;

use service_specs::services::get_all_service_configs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let services = get_all_service_configs();

    for service in services {
        println!("{}", service);
    }

    Ok(())
}
