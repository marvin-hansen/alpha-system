use zenoh::config::Config;

pub const KEY_EXPRESS: &str = "key/expression";
pub const LOW_LATENCY: bool = false;
pub const MAX_MESSAGES: i32 = 1_000_000;


pub fn get_config(low_latency: bool) -> Config{
    let mut config= Config::default();

    return if low_latency {
        config.transport.qos.set_enabled(false).expect("Failed to disable QoS ");
        config.transport.unicast.set_lowlatency(true).expect("Failed to set low latency mode");
        config
    } else {
        config
    }
}
