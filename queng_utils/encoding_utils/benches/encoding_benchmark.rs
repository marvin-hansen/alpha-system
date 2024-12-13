use criterion::{criterion_group, criterion_main, Criterion};
use encoding_utils::string_int_encoding::{int_to_str, str_to_int};

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
            b.iter(|| str_to_int(test_str))
        });
    }
    group.finish();
}

fn bench_int_to_str(c: &mut Criterion) {
    let test_cases = [
        str_to_int("A").unwrap(),
        str_to_int("ABC").unwrap(),
        str_to_int("TEST_123").unwrap(),
        str_to_int("ABCDEFGHIJ").unwrap(),
    ];

    let mut group = c.benchmark_group("int_to_str");
    for (i, &test_int) in test_cases.iter().enumerate() {
        group.bench_function(format!("decode_len_{}", i + 1), |b| {
            b.iter(|| int_to_str(test_int))
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
                let encoded = str_to_int(test_str).unwrap();
                int_to_str(encoded).unwrap()
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_str_to_int, bench_int_to_str, bench_roundtrip);
criterion_main!(benches);
