use criterion::criterion_main;

mod ring_buffer;

criterion_main! {
ring_buffer::bit_map_benchmark::bitmap,
ring_buffer::sequence_bench::sequence,
ring_buffer::ring_buffer_bench::ring_buffer,
}
