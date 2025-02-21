/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use criterion::{Criterion, criterion_group};

use sliding_window::{ArrayStorage, SlidingWindow};

use crate::window_type::fields::{CAPACITY, SIZE};

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

fn get_sliding_window() -> SlidingWindow<ArrayStorage<Data, SIZE, CAPACITY>, Data> {
    sliding_window::new_with_array_storage()
}

fn array_backed_benchmark(criterion: &mut Criterion) {
    let mut w = get_sliding_window();
    criterion.bench_function("array_push", |bencher| {
        bencher.iter(|| w.push(Data { dats: 0 }))
    });
}

criterion_group! {
    name = window_array_backed;
    config = Criterion::default().sample_size(100);
    targets =
    array_backed_benchmark,
}
