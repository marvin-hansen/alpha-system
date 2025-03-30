/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use criterion::criterion_main;

mod window_type;

criterion_main! {
     window_type::bench_window_arr::window_array_backed,
     window_type::bench_window_vec::window_vector_backed,
     window_type::bench_window_comp::window_impl_comp,
    // Unsafe-specific benchmarks
     window_type::bench_window_unsafe_arr::window_unsafe_array_backed,
     window_type::bench_window_unsafe_vec::window_unsafe_vector_backed
}
