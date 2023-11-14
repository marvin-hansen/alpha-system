pub fn time_execution<T, F: FnOnce() -> T>(f: F, f_name: &str) -> T {
    let start = std::time::Instant::now();
    let res = f();
    println!(
        "{} Execution took {:?} ",
        f_name.to_uppercase(),
        start.elapsed()
    );
    res
}