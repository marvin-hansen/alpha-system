use criterion::{black_box, criterion_group, criterion_main, Criterion};
use encoding_utils::string_int128_codec::{decode_str_to_int128, encode_int128_to_str};

fn bench_encode_int128(c: &mut Criterion) {
    let test_values = [
        (u128::MAX, "ZZZZZZZZZZZZZZZZZZZZ"),
        (0u128, ""),
        (123456789u128, "3D2MP7"),
    ];

    let mut group = c.benchmark_group("int128_encode");
    for (input, _) in test_values.iter() {
        group.bench_function(format!("encode_{}", input), |b| {
            b.iter(|| {
                let mut output = String::with_capacity(32);
                encode_int128_to_str(black_box(*input), &mut output).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_decode_int128(c: &mut Criterion) {
    let test_values = [
        ("ZZZZZZZZZZZZZZZZZZZZ", u128::MAX),
        ("", 0u128),
        ("3D2MP7", 123456789u128),
    ];

    let mut group = c.benchmark_group("int128_decode");
    for (input, _) in test_values.iter() {
        group.bench_function(format!("decode_{}", input), |b| {
            b.iter(|| {
                decode_str_to_int128(black_box(input)).unwrap();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_encode_int128, bench_decode_int128);
criterion_main!(benches);
