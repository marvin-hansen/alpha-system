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

# Configure Rust Toolchain to use.
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.76.0",
    ],
)

# Track dependencies of all crates.
# When you add a new crate, re-run:
# CARGO_BAZEL_REPIN=true bazel sync --only=crate_index
load("@rules_rust//crate_universe:defs.bzl", "crates_repository")
crates_repository(
    name = "crates",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:cargo-bazel-lock",
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
        "//:queng_services/qdgw/Cargo.toml",
        "//:queng_services/smdb/Cargo.toml",
        "//:queng_services/symdb/Cargo.toml",
        "//:queng_services/vex/Cargo.toml",
        "//:queng_specs/db_specs/Cargo.toml",
        "//:queng_specs/exchange_specs/Cargo.toml",
        "//:queng_specs/file_specs/Cargo.toml",
        "//:queng_specs/service_specs/Cargo.toml",
        "//:queng_utils/client_utils/Cargo.toml",
        "//:queng_utils/proto_utils/Cargo.toml",
        "//:queng_utils/service_utils/Cargo.toml",
    ],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies()

load("@rules_rust//proto/protobuf:repositories.bzl", "rust_proto_protobuf_dependencies", "rust_proto_protobuf_register_toolchains")
rust_proto_protobuf_dependencies()
rust_proto_protobuf_register_toolchains()

# Load gazelle_rust. In a real project, this would use http_archive.
GAZELLE_RUST_COMMIT = "aef7695c4a9b6c3e32255ed48570e62199f52537"
GAZELLE_RUST_SHA256 = ""

http_archive(
    name = "gazelle_rust",
    sha256 = GAZELLE_RUST_SHA256,
    strip_prefix = "gazelle_rust-{}".format(GAZELLE_RUST_COMMIT),
    url = "https://github.com/Calsign/gazelle_rust/archive/{}.zip".format(GAZELLE_RUST_COMMIT),
)

# Load gazelle_rust transitive dependencies (includes gazelle). You can also load gazelle yourself,
# before these macros.

load("@gazelle_rust//:deps1.bzl", "gazelle_rust_dependencies1")

gazelle_rust_dependencies1()

load("@gazelle_rust//:deps2.bzl", "gazelle_rust_dependencies2")

gazelle_rust_dependencies2()