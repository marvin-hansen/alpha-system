mod ci;

pub(crate) mod shared;

pub mod errors;
pub mod prelude;

pub struct EnvUtil {
    clickhouse_container_name: String,
    clickhouse_container_port: u16,
    dbg: bool,
}

impl EnvUtil {
    pub fn new() -> Self {
        Self::build(false)
    }

    pub fn with_debug() -> Self {
        Self::build(true)
    }

    fn build(dbg: bool) -> Self {
        Self {
            clickhouse_container_name: String::new(),
            clickhouse_container_port: 0,
            dbg,
        }
    }
}

impl EnvUtil {
    pub fn set_clickhouse_container_name(&mut self, clickhouse_container_name: String) {
        self.clickhouse_container_name = clickhouse_container_name;
    }
    pub fn set_clickhouse_container_port(&mut self, clickhouse_container_port: u16) {
        self.clickhouse_container_port = clickhouse_container_port;
    }
    pub fn clickhouse_container_name(&self) -> &str {
        &self.clickhouse_container_name
    }
    pub fn clickhouse_container_port(&self) -> u16 {
        self.clickhouse_container_port
    }
}

impl EnvUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[EnvUtil]: {}", s);
        }
    }
}
