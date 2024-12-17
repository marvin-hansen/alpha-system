use criterion::{criterion_group, criterion_main, Criterion};
use encoding_utils::string_int64_encoding::{decode_int64_to_str, encode_str_to_int64};

fn bench_str_to_int(c: &mut Criterion) {
    let test_cases = [
        "A",          // Single char
        "ABC",        // Short string
        "TEST_123",   // Medium string
        "ABCDEFGHIJ", // Max length string
    ];

    let mut group = c.benchmark_group("str_to_int");
    for test_str in test_cases {
        group.bench_function(format!("encode_{}", test_str), |b| {
            b.iter(|| encode_str_to_int64(test_str))
        });
    }
    group.finish();
}

fn bench_int_to_str(c: &mut Criterion) {
    let test_cases = [
        encode_str_to_int64("A").unwrap(),
        encode_str_to_int64("ABC").unwrap(),
        encode_str_to_int64("TEST_123").unwrap(),
        encode_str_to_int64("ABCDEFGHIJ").unwrap(),
    ];

    let mut group = c.benchmark_group("int_to_str");
    for (i, &test_int) in test_cases.iter().enumerate() {
        group.bench_function(format!("decode_len_{}", i + 1), |b| {
            b.iter(|| decode_int64_to_str(test_int))
        });
    }
    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let test_cases = ["A", "ABC", "TEST_123", "ABCDEFGHIJ"];

    let mut group = c.benchmark_group("roundtrip");
    for test_str in test_cases {
        group.bench_function(format!("roundtrip_{}", test_str), |b| {
            b.iter(|| {
                let encoded = encode_str_to_int64(test_str).unwrap();
                decode_int64_to_str(encoded).unwrap()
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_str_to_int, bench_int_to_str, bench_roundtrip);
criterion_main!(benches);
