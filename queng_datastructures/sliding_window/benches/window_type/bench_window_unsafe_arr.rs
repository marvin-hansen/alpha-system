use criterion::criterion_group;

use criterion::Criterion;

use sliding_window::{SlidingWindow, UnsafeArrayStorage};

use crate::window_type::fields::{CAPACITY, SIZE};

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

fn get_sliding_window() -> SlidingWindow<UnsafeArrayStorage<Data, SIZE, CAPACITY>, Data> {
    sliding_window::new_with_unsafe_array_storage()
}

fn array_backed_benchmark(criterion: &mut Criterion) {
    let mut w = get_sliding_window();
    criterion.bench_function("unsafe_array_push", |bencher| {
        bencher.iter(|| w.push(Data { dats: 0 }))
    });
}

criterion_group! {
    name = window_unsafe_array_backed;
    config = Criterion::default().sample_size(100);
    targets =
    array_backed_benchmark,
}
