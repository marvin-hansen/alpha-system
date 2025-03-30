use hdrhistogram::Histogram;

pub(crate) fn report_latency(hist: &Histogram<u64>) {
    // Calculate mean
    let mean = hist.mean();

    // Calculate variance
    let mut variance = 0.0;

    // Iterate through recorded values
    for recorded_value in hist.iter_recorded() {
        let value = recorded_value.value_iterated_to();
        let count = recorded_value.count_at_value();

        let diff = (value as f64) - mean;
        variance += diff * diff * (count as f64);
    }

    // Calculate standard deviation
    let std_dev = variance.sqrt();

    println!("Latency Histogram:");
    println!("Min:    {} ns", hist.min());
    println!("Max:    {} ns", hist.max());
    println!("Mean:    {:.2} ns", mean);
    println!("Std Dev: {:.2} ns", std_dev);
    println!("p50:     {} ns", hist.value_at_quantile(0.50));
    println!("p90:     {} ns", hist.value_at_quantile(0.90));
    println!("p99:     {} ns", hist.value_at_quantile(0.99));
    println!("p99.9:   {} ns", hist.value_at_quantile(0.999));
    println!("p99.99:  {} ns", hist.value_at_quantile(0.9999));
}

// // calculates the throughput in gigabytes per second (GB/sec)
// pub(crate) fn report_throughput(bytes: u64, duration: std::time::Duration) {
//     let throughput = (bytes as f64) / duration.as_secs_f64() / 1_000_000_000.0f64;
//     println!("Throughput: {:.2} Gb/sec", throughput);
// }
