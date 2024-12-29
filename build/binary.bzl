load("@rules_rust//rust:defs.bzl", "rust_binary")

# Use this to build optimized binaries that are pure rust
def build_binary_opt(name, srcs, deps = [], visibility = ["//visibility:public"]):
    # Build optimized Rust binary
    rust_binary(
        name = name,
        srcs = srcs,
        crate_root = "src/main.rs",
        rustc_flags = select({
            "//:release": [
                "-Clto=true",
                "-Ccodegen-units=1",
                "-Cpanic=abort",
                "-Copt-level=3",
                "-Cstrip=symbols",
                # "-Ctarget-cpu=native", # Only use this when the build CPU is the same as the target CPU
            ],
            "//conditions:default": [
                "-Copt-level=0",
            ],
        }),
        tags = [
            name,
            "binary",
        ],
        deps = deps,
        visibility = visibility,
    )

# Use this to build optimized binaries with optimization across FFI due to C dependency i.e. libpg
def build_rust_ffi_binary_opt(name, srcs, deps = [], visibility = ["//visibility:public"]):
    # Build optimized Rust binary
    rust_binary(
        name = name,
        srcs = srcs,
        crate_root = "src/main.rs",
        rustc_flags = select({
            "//:release": [
                # https://users.rust-lang.org/t/is-the-build-flag-linker-plugin-lto-still-needed/56352
                "-Clink-arg=-flto",
                "-Ccodegen-units=1",
                "-Cpanic=abort",
                "-Copt-level=3",
                "-Cstrip=symbols",
                # "-Ctarget-cpu=native", # Only use this when the build CPU is the same as the target CPU
            ],
            "//conditions:default": [
                "-Copt-level=0",
            ],
        }),
        tags = [
            name,
            "binary",
        ],
        deps = deps,
        visibility = visibility,
    )