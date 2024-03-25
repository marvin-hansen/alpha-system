workspace(name = "queng")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "cd55a062e763b9349921f0f5db8c3933288dc8ba4f76dd9416aac68acee3cb94",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.5.0/bazel-skylib-1.5.0.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.5.0/bazel-skylib-1.5.0.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
bazel_skylib_workspace()

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "rules_rust",
    integrity = "sha256-ww398ehv1QZQp26mRbOkXy8AZnsGGHpoXpVU4WfKl+4=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.40.0/rules_rust-v0.40.0.tar.gz"],
)

###############################################################################
# R U L E S  R U S T
###############################################################################

rust_version = "1.76.0"

# Configure Rust Toolchain to use.
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains(
    edition = "2021",
    versions = [
        rust_version,
    ],
)

load("@rules_rust//proto/prost/private:repositories.bzl", "rust_prost_dependencies", "rust_prost_register_toolchains")
rust_prost_dependencies()
rust_prost_register_toolchains()

load("@rules_rust//proto/prost:transitive_repositories.bzl", "rust_prost_transitive_repositories")
rust_prost_transitive_repositories()

###############################################################################
# R U L E S  P R O T O
###############################################################################
# https://github.com/bazelbuild/rules_proto/releases
http_archive(
    name = "rules_proto",
    sha256 = "dc3fb206a2cb3441b485eb1e423165b231235a1ea9b031b4433cf7bc1fa460dd",
    strip_prefix = "rules_proto-5.3.0-21.7",
    urls = [
        "https://github.com/bazelbuild/rules_proto/archive/refs/tags/5.3.0-21.7.tar.gz",
    ],
)
load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")
rules_proto_dependencies()
rules_proto_toolchains()

###############################################################################
# R U S T  C R A T E S
###############################################################################

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies(bootstrap = True)

# Track dependencies of all crates.
# When you add a new crate, re-run:
# CARGO_BAZEL_REPIN=true bazel sync --only=crate_index
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")
crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    generator = "@cargo_bazel_bootstrap//:cargo-bazel",
    lockfile = "//:cargo-bazel-lock.json",
    manifests = [
        "//:Cargo.toml",
        "//:queng_cli/data_importer/Cargo.toml",
        "//:queng_cli/smdb_manager/Cargo.toml",
        "//:queng_cli/spec_manager/Cargo.toml",
        "//:queng_clients/dbgw_client/Cargo.toml",
        "//:queng_clients/ims_data_client/Cargo.toml",
        "//:queng_clients/rest_client/Cargo.toml",
        "//:queng_clients/symdb_client/Cargo.toml",
        "//:queng_common/Cargo.toml",
        "//:queng_components/config_manager/Cargo.toml",
        "//:queng_components/ctx_manager/Cargo.toml",
        "//:queng_components/db_query_manager/Cargo.toml",
        "//:queng_components/db_surreal_manager/Cargo.toml",
        "//:queng_components/dns_manager/Cargo.toml",
        "//:queng_components/pattern_manager/Cargo.toml",
        "//:queng_components/symbol_manager/Cargo.toml",
        "//:queng_proto/Cargo.toml",
        "//:queng_providers/cmdb_provider/Cargo.toml",
        "//:queng_providers/smdb_provider/Cargo.toml",
        "//:queng_sbe/bindings/Cargo.toml",
        "//:queng_sbe/sbe_messages/Cargo.toml",
        "//:queng_services/cmdb/Cargo.toml",
        "//:queng_services/dbgw/Cargo.toml",
        "//:queng_services/ims/data/binance_data/Cargo.toml",
        "//:queng_services/ims/data/shared_data/Cargo.toml",
        "//:queng_services/mdm/Cargo.toml",
        "//:queng_services/smdb/Cargo.toml",
        "//:queng_services/symdb/Cargo.toml",
        "//:queng_specs/db_specs/Cargo.toml",
        "//:queng_specs/exchange_specs/Cargo.toml",
        "//:queng_specs/file_specs/Cargo.toml",
        "//:queng_specs/service_specs/Cargo.toml",
        "//:queng_utils/client_utils/Cargo.toml",
        "//:queng_utils/proto_utils/Cargo.toml",
        "//:queng_utils/service_utils/Cargo.toml",
    ],
    # Solves: error[E0433]: failed to resolve: could not find `visit` in `syn`
    # https://stackoverflow.com/questions/60576277/yanked-subdependency-error-e0433-failed-to-resolve-could-not-find-rt-i
       packages = {
            "wasm-bindgen-macro-support": crate.spec(
                version = "0.2.90",
            ),
        },
)

load("@crate_index//:defs.bzl", "crate_repositories")
crate_repositories()
