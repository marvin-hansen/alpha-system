/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use encoding_utils::{decode_int64_to_str, encode_str_to_int64};

fn bench_encode_str_to_int64(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_str_to_int64");

    // Empty string
    group.bench_function("empty", |b| b.iter(|| encode_str_to_int64(black_box(""))));

    // Single character
    group.bench_function("single_char", |b| {
        b.iter(|| encode_str_to_int64(black_box("A")))
    });

    // Short string (3 chars)
    group.bench_function("short_string", |b| {
        b.iter(|| encode_str_to_int64(black_box("ABC")))
    });

    // Medium string (6 chars)
    group.bench_function("medium_string", |b| {
        b.iter(|| encode_str_to_int64(black_box("ABC123")))
    });

    // Long string (10 chars, maximum length)
    group.bench_function("max_length", |b| {
        b.iter(|| encode_str_to_int64(black_box("ABCDEFGHIJ")))
    });

    // Mixed characters
    group.bench_function("mixed_chars", |b| {
        b.iter(|| encode_str_to_int64(black_box("Ab1_Cd2_Ef")))
    });

    group.finish();
}

fn bench_decode_int64_to_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_int64_to_str");

    // Empty string (encoded)
    let empty_encoded = encode_str_to_int64("").unwrap();
    group.bench_function("empty", |b| {
        b.iter(|| decode_int64_to_str(black_box(empty_encoded)))
    });

    // Single character (encoded)
    let single_encoded = encode_str_to_int64("A").unwrap();
    group.bench_function("single_char", |b| {
        b.iter(|| decode_int64_to_str(black_box(single_encoded)))
    });

    // Short string (3 chars, encoded)
    let short_encoded = encode_str_to_int64("ABC").unwrap();
    group.bench_function("short_string", |b| {
        b.iter(|| decode_int64_to_str(black_box(short_encoded)))
    });

    // Medium string (6 chars, encoded)
    let medium_encoded = encode_str_to_int64("ABC123").unwrap();
    group.bench_function("medium_string", |b| {
        b.iter(|| decode_int64_to_str(black_box(medium_encoded)))
    });

    // Long string (10 chars, maximum length, encoded)
    let max_encoded = encode_str_to_int64("ABCDEFGHIJ").unwrap();
    group.bench_function("max_length", |b| {
        b.iter(|| decode_int64_to_str(black_box(max_encoded)))
    });

    // Mixed characters (encoded)
    let mixed_encoded = encode_str_to_int64("Ab1_Cd2_Ef").unwrap();
    group.bench_function("mixed_chars", |b| {
        b.iter(|| decode_int64_to_str(black_box(mixed_encoded)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_encode_str_to_int64,
    bench_decode_int64_to_str
);
criterion_main!(benches);
