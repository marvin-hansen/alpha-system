use criterion::{black_box, criterion_group, criterion_main, Criterion};
use encoding_utils::{decode_int128_to_str, encode_str_to_int128};

fn bench_encode_str_to_int128(c: &mut Criterion) {
    let mut group = c.benchmark_group("int128_encode");

    // Empty string
    group.bench_function("empty", |b| b.iter(|| encode_str_to_int128(black_box(""))));

    // Single character
    group.bench_function("single_char", |b| {
        b.iter(|| encode_str_to_int128(black_box("A")))
    });

    // Short string (3 chars)
    group.bench_function("short_string", |b| {
        b.iter(|| encode_str_to_int128(black_box("ABC")))
    });

    // Medium string (6 chars)
    group.bench_function("medium_string", |b| {
        b.iter(|| encode_str_to_int128(black_box("ABC123")))
    });

    // Long string (15 chars)
    group.bench_function("long_string", |b| {
        b.iter(|| encode_str_to_int128(black_box("ABCDEFGHIJKLMNO")))
    });

    // Maximum length string (20 chars)
    group.bench_function("max_length", |b| {
        b.iter(|| encode_str_to_int128(black_box("ABCDEFGHIJKLMNOPQRST")))
    });

    // Mixed characters
    group.bench_function("mixed_chars", |b| {
        b.iter(|| encode_str_to_int128(black_box("Ab1_Cd2_Ef3_Gh4_Ij")))
    });

    group.finish();
}

fn bench_decode_int128_to_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("int128_decode");

    // Empty string (encoded)
    let empty_encoded = encode_str_to_int128("").unwrap();
    group.bench_function("empty", |b| {
        b.iter(|| decode_int128_to_str(black_box(empty_encoded)))
    });

    // Single character (encoded)
    let single_encoded = encode_str_to_int128("A").unwrap();
    group.bench_function("single_char", |b| {
        b.iter(|| decode_int128_to_str(black_box(single_encoded)))
    });

    // Short string (3 chars, encoded)
    let short_encoded = encode_str_to_int128("ABC").unwrap();
    group.bench_function("short_string", |b| {
        b.iter(|| decode_int128_to_str(black_box(short_encoded)))
    });

    // Medium string (6 chars, encoded)
    let medium_encoded = encode_str_to_int128("ABC123").unwrap();
    group.bench_function("medium_string", |b| {
        b.iter(|| decode_int128_to_str(black_box(medium_encoded)))
    });

    // Long string (15 chars, encoded)
    let long_encoded = encode_str_to_int128("ABCDEFGHIJKLMNO").unwrap();
    group.bench_function("long_string", |b| {
        b.iter(|| decode_int128_to_str(black_box(long_encoded)))
    });

    // Maximum length string (20 chars, encoded)
    let max_encoded = encode_str_to_int128("ABCDEFGHIJKLMNOPQRST").unwrap();
    group.bench_function("max_length", |b| {
        b.iter(|| decode_int128_to_str(black_box(max_encoded)))
    });

    // Mixed characters (encoded)
    let mixed_encoded = encode_str_to_int128("Ab1_Cd2_Ef3_Gh4_Ij").unwrap();
    group.bench_function("mixed_chars", |b| {
        b.iter(|| decode_int128_to_str(black_box(mixed_encoded)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_encode_str_to_int128,
    bench_decode_int128_to_str
);
criterion_main!(benches);
