use criterion::{black_box, criterion_group, criterion_main, Criterion};
use encoding_utils::{int_to_str, str_to_int};

fn benchmark_encoding(c: &mut Criterion) {
    let test_strings = ["", "a", "hello", "12345678"];

    let mut group = c.benchmark_group("string_encoding");

    for &s in &test_strings {
        group.bench_function(format!("str_to_int_{}", s), |b| {
            b.iter(|| str_to_int(black_box(s)))
        });
    }

    for &s in &test_strings {
        if let Some(encoded) = str_to_int(s) {
            group.bench_function(format!("int_to_str_{}", s), |b| {
                b.iter(|| int_to_str(black_box(encoded)))
            });
        }
    }

    group.finish();
}

criterion_group!(benches, benchmark_encoding);
criterion_main!(benches);
