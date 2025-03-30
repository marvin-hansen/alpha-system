use crate::benchmarks::utils;
use alpha_socket::RingBuffer;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group};
use hdrhistogram::Histogram;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 1024 * 1024; // 1 MB buffer size

fn benchmark_write(c: &mut Criterion) {
    let path = "bench_ring_buffer_write";

    let mut buffer =
        RingBuffer::new(path, Some(BUFFER_SIZE)).expect("Failed to create ring buffer");
    let mut read_buf = vec![0u8; 128];

    c.bench_with_input(
        BenchmarkId::new(" write 128 bytes", "single"),
        &(),
        |b, _| {
            let mut hist = Histogram::<u64>::new(3).unwrap();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();

                for _ in 0..iters {
                    let start = std::time::Instant::now();
                    // Write into the buffer
                    buffer
                        .write_all(black_box(&read_buf))
                        .expect("Failed to read from ring buffer");

                    // Log the latency
                    hist.record(start.elapsed().as_nanos() as u64).unwrap();

                    // Read to clear the buffer otherwise the buffer will return a WouldBlock error when its full.
                    buffer
                        .read_exact(black_box(&mut read_buf))
                        .expect("Failed to read from ring buffer");
                }
                start.elapsed()
            });

            // Report histogram after all iterations
            utils::report_latency(&hist);
        },
    );
}

fn benchmark_read(c: &mut Criterion) {
    let path = "bench_ring_buffer_read";

    let mut buffer =
        RingBuffer::new(path, Some(BUFFER_SIZE)).expect("Failed to create ring buffer");
    let mut read_buf = vec![0u8; 128];

    c.bench_with_input(
        BenchmarkId::new(" write 128 bytes", "single"),
        &(),
        |b, _| {
            let mut hist = Histogram::<u64>::new(3).unwrap();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Write into the buffer
                    buffer
                        .write_all(black_box(&read_buf))
                        .expect("Failed to read from ring buffer");

                    let start = std::time::Instant::now();

                    // Read from the buffer
                    buffer
                        .read_exact(black_box(&mut read_buf))
                        .expect("Failed to read from ring buffer");

                    // Log the latency
                    hist.record(start.elapsed().as_nanos() as u64).unwrap();
                }
                start.elapsed()
            });

            // Report histogram after all iterations
            utils::report_latency(&hist);
        },
    );
}

fn benchmark_read_write(c: &mut Criterion) {
    let path = "bench_ring_buffer_read_write";

    let mut buffer =
        RingBuffer::new(path, Some(BUFFER_SIZE)).expect("Failed to create ring buffer");
    let mut read_buf = vec![0u8; 128];

    c.bench_with_input(
        BenchmarkId::new("read and write 128 bytes", "single"),
        &(),
        |b, _| {
            let mut hist = Histogram::<u64>::new(3).unwrap();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let start = std::time::Instant::now();
                    // Write into the buffer
                    buffer
                        .write_all(black_box(&read_buf))
                        .expect("Failed to read from ring buffer");
                    // Read from the buffer
                    buffer
                        .read_exact(black_box(&mut read_buf))
                        .expect("Failed to read from ring buffer");

                    // Log the latency
                    hist.record(start.elapsed().as_nanos() as u64).unwrap();
                }
                start.elapsed()
            });

            // Report histogram after all iterations
            utils::report_latency(&hist);
        },
    );
}

criterion_group!(
    ring_buffer_benches,
    benchmark_write,
    benchmark_read,
    benchmark_read_write
);
