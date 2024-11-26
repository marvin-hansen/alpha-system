use criterion::criterion_main;

mod window_type;

// Always run these benchmarks
#[cfg(not(feature = "unsafe"))]
criterion_main! {
    window_type::bench_window_arr::window_array_backed,
    window_type::bench_window_vec::window_vector_backed,
    window_type::bench_window_comp::window_impl_comp
}

// Run these benchmarks only when unsafe feature is enabled

criterion_main! {
     window_type::bench_window_arr::window_array_backed,
     window_type::bench_window_vec::window_vector_backed,
     window_type::bench_window_comp::window_impl_comp,
    // Unsafe-specific benchmarks
     window_type::bench_window_unsafe_arr::window_unsafe_array_backed,
     window_type::bench_window_unsafe_vec::window_unsafe_vector_backed
}
