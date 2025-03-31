use crate::benchmarks::utils;
use alpha_socket::AlphaSocket;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use hdrhistogram::Histogram;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

const DATA_SIZE: usize = 128; // 128 bytes per iteration

fn benchmark_unix_stream_latency(c: &mut Criterion) {
    let (mut client_stream, mut server_stream) = UnixStream::pair().unwrap();
    let mut read_buf = vec![0u8; DATA_SIZE];

    c.bench_with_input(
        BenchmarkId::new("Unix Socket Latency: Write 128 Bytes", "single"),
        &(),
        |b, _| {
            let mut hist = Histogram::<u64>::new(3).unwrap();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();

                for _ in 0..iters {
                    let start = std::time::Instant::now();
                    // Write into the buffer
                    client_stream
                        .write_all(black_box(&read_buf))
                        .expect("Failed to read from UnixStream");

                    // Log the latency
                    hist.record(start.elapsed().as_nanos() as u64).unwrap();

                    // Read to clear the buffer otherwise the buffer will return a WouldBlock error when its full.
                    server_stream
                        .read_exact(black_box(&mut read_buf))
                        .expect("Failed to read from UnixStream");
                }
                start.elapsed()
            });

            // Report histogram after all iterations
            utils::report_latency(&hist);
        },
    );
}

fn benchmark_alpha_stream_latency(c: &mut Criterion) {
    let (mut client_stream, mut server_stream) = AlphaSocket::pair().unwrap();
    let mut read_buf = vec![0u8; DATA_SIZE];

    c.bench_with_input(
        BenchmarkId::new("Alpha Socket Latency: Write 128 Bytes", "single"),
        &(),
        |b, _| {
            let mut hist = Histogram::<u64>::new(3).unwrap();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();

                for _ in 0..iters {
                    let start = std::time::Instant::now();
                    // Write into the buffer
                    client_stream
                        .write_all(black_box(&read_buf))
                        .expect("Failed to read from AlphaStream");

                    // Log the latency
                    hist.record(start.elapsed().as_nanos() as u64).unwrap();

                    // Read to clear the buffer otherwise the buffer will return a WouldBlock error when its full.
                    server_stream
                        .read_exact(black_box(&mut read_buf))
                        .expect("Failed to read from AlphaStream");
                }
                start.elapsed()
            });

            // Report histogram after all iterations
            utils::report_latency(&hist);
        },
    );
}

criterion_group!(
    socket_benches,
    benchmark_unix_stream_latency,
    benchmark_alpha_stream_latency
);
criterion_main!(socket_benches);
