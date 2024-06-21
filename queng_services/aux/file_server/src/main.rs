use jemallocator::Jemalloc;
use warp::Filter;

mod errors;
mod fields;
mod handler;
mod init;
mod run;
mod types;
mod utils;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const VRB: bool = false;
const PORT: u16 = 7777;

#[tokio::main]
async fn main() {
    run::run().await
}
