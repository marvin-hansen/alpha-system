use criterion::{criterion_group, Criterion};

use sliding_window::{SlidingWindow, VectorStorage};

use crate::window_type::fields::{MULT, SIZE};

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

fn get_sliding_window() -> SlidingWindow<VectorStorage<Data>, Data> {
    sliding_window::new_with_vector_storage(SIZE, MULT)
}

fn vector_backed_benchmark(criterion: &mut Criterion) {
    let mut w = get_sliding_window();
    criterion.bench_function("vector_push", |bencher| {
        bencher.iter(|| w.push(Data { dats: 0 }))
    });
}

criterion_group! {
    name = window_vector_backed;
    config = Criterion::default().sample_size(100);
    targets =
    vector_backed_benchmark,
}
