mod benchmarks;

use crate::benchmarks::ring_buffer_bench;
use crate::benchmarks::socket_bench;
use criterion::criterion_main;

criterion_main!(
    ring_buffer_bench::ring_buffer_benches,
    socket_bench::socket_benches
);
