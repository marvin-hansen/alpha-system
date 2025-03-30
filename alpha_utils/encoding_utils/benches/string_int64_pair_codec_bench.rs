/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use encoding_utils::{decode_pair_64_to_str, encode_str_to_pair_u64};

pub fn pair_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("pair_u64_encode");

    // Benchmark empty string
    group.bench_function("encode_empty", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box("")))
    });

    // Single character
    group.bench_function("encode_single", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box("A")))
    });

    // Short string (3 chars)
    group.bench_function("encode_short", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box("ABC")))
    });

    // Benchmark medium length string (10 chars)
    group.bench_function("encode_medium", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box("ABC123")))
    });

    // Long string (16 chars)
    group.bench_function("encode_long", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box("ABCDEFGHIJ0123456789")))
    });

    // Benchmark max length string (20 chars, maximum length)
    let max_str = "ABCDEFGHIJ0123456789";
    group.bench_function("encode_max_length", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box(max_str)))
    });

    // Mixed characters
    group.bench_function("encode_mixed", |b| {
        b.iter(|| encode_str_to_pair_u64(black_box("Ab1_Cd2_Ef")))
    });

    group.finish();
}

pub fn pair_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("pair_u64_decode");

    // Benchmark empty string
    let empty_encoded = encode_str_to_pair_u64("").unwrap();
    group.bench_function("decode_empty", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(empty_encoded)))
    });

    // Single character
    let single_encoded = encode_str_to_pair_u64("A").unwrap();
    group.bench_function("decode_single", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(single_encoded)))
    });

    // Short string (3 chars)
    let short_encoded = encode_str_to_pair_u64("ABC").unwrap();
    group.bench_function("decode_short", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(short_encoded)))
    });

    // Benchmark medium length string (10 chars)
    let medium_encoded = encode_str_to_pair_u64("ABC123").unwrap();
    group.bench_function("decode_medium", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(medium_encoded)))
    });

    // Long string (16 chars)
    let max_encoded = encode_str_to_pair_u64("ABCDEFGHIJ0123456789").unwrap();
    group.bench_function("decode_long", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(max_encoded)))
    });

    // Benchmark max length string (20 chars, maximum length)
    let max_str = "ABCDEFGHIJ0123456789";
    let max_encoded = encode_str_to_pair_u64(max_str).unwrap();
    group.bench_function("decode_max", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(max_encoded)))
    });

    // Mixed characters
    let mixed_encoded = encode_str_to_pair_u64("Ab1_Cd2_Ef").unwrap();
    group.bench_function("decode_mixed", |b| {
        b.iter(|| decode_pair_64_to_str(black_box(mixed_encoded)))
    });

    group.finish();
}

criterion_group!(benches, pair_encode, pair_decode);
criterion_main!(benches);
