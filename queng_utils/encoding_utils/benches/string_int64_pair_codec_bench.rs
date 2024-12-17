use criterion::{black_box, criterion_group, criterion_main, Criterion};
use encoding_utils::{decode_pair_64_to_str, encode_str_to_pair_u64};

pub fn pair_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("pair_u64_encode");

    // Benchmark max length string
    let max_str = "ABCDEFGHIJ0123456789";
    group.bench_function("encode_max", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box(max_str)))
    });

    // Benchmark empty string
    group.bench_function("encode_empty", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box("")))
    });

    // Benchmark medium length string
    let medium_str = "Hello_World123";
    group.bench_function("encode_medium", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box(medium_str)))
    });

    group.finish();
}

pub fn pair_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("pair_u64_decode");

    // Benchmark max length string
    let max_encoded = encode_str_to_pair_u64("ABCDEFGHIJ0123456789").unwrap();
    group.bench_function("decode_max", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(max_encoded)))
    });

    // Benchmark empty string
    let empty_encoded = encode_str_to_pair_u64("").unwrap();
    group.bench_function("decode_empty", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(empty_encoded)))
    });

    // Benchmark medium length string
    let medium_encoded = encode_str_to_pair_u64("Hello_World123").unwrap();
    group.bench_function("decode_medium", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(medium_encoded)))
    });

    group.finish();
}

criterion_group!(benches, pair_encode, pair_decode);
criterion_main!(benches);
