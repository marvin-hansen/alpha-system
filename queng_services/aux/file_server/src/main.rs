use mimalloc::MiMalloc;

mod errors;
mod fields;
mod handler;
mod init;
mod run;
mod types;
mod utils;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const VRB: bool = false;
const PORT: u16 = 7777;

#[tokio::main]
async fn main() {
    run::run().await
}
