workspace(name = "queng")
# rule http_archive
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

###############################################################################
# R U L E S  S K Y L I B
# Releases: https://github.com/bazelbuild/bazel-skylib/releases
###############################################################################
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

###############################################################################
# R U L E S  R U S T
# Releases: # https://github.com/bazelbuild/rules_rust/releases
###############################################################################
http_archive(
    name = "rules_rust",
    integrity = "sha256-mUV3N2A8ORVVZbrm3O9yepAe/Kv4MD2ob9YQhB8aOI8=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.41.1/rules_rust-v0.41.1.tar.gz"],
)

RUST_EDITION = "2021"
RUST_VERSION = "1.77.1"

# Configure Rust Toolchain to use.
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains", "rust_repository_set")
rules_rust_dependencies()
rust_register_toolchains(
    edition = RUST_EDITION,
    versions = [
        RUST_VERSION,
    ],
)

###############################################################################
# R U L E S  A S P E C T
# Releases: https://github.com/aspect-build/bazel-lib/releases
###############################################################################
http_archive(
    name = "aspect_bazel_lib",
    sha256 = "ac6392cbe5e1cc7701bbd81caf94016bae6f248780e12af4485d4a7127b4cb2b",
    strip_prefix = "bazel-lib-2.6.1",
    url = "https://github.com/aspect-build/bazel-lib/releases/download/v2.6.1/bazel-lib-v2.6.1.tar.gz",
)

load("@aspect_bazel_lib//lib:repositories.bzl", "aspect_bazel_lib_dependencies", "aspect_bazel_lib_register_toolchains")
aspect_bazel_lib_dependencies()
aspect_bazel_lib_register_toolchains()

################################################################################
# R U L E S  M U L T I R U N
# Releases: https://github.com/keith/rules_multirun/releases
################################################################################
http_archive(
    name = "rules_multirun",
    sha256 = "0e124567fa85287874eff33a791c3bbdcc5343329a56faa828ef624380d4607c",
    url = "https://github.com/keith/rules_multirun/releases/download/0.9.0/rules_multirun.0.9.0.tar.gz",
)

###############################################################################
# R U L E S  P R O T O
# Releases: https://github.com/bazelbuild/rules_proto/releases
###############################################################################
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

load("@rules_rust//proto/prost/private:repositories.bzl", "rust_prost_dependencies", "rust_prost_register_toolchains")
rust_prost_dependencies()
rust_prost_register_toolchains()

load("@rules_rust//proto/prost:transitive_repositories.bzl", "rust_prost_transitive_repositories")
rust_prost_transitive_repositories()

###############################################################################
# R U L E S  O C I  I M A G E
# Releases: https://github.com/bazel-contrib/rules_oci/releases
###############################################################################
http_archive(
    name = "rules_oci",
    sha256 = "56d5499025d67a6b86b2e6ebae5232c72104ae682b5a21287770bd3bf0661abf",
    strip_prefix = "rules_oci-1.7.5",
    url = "https://github.com/bazel-contrib/rules_oci/releases/download/v1.7.5/rules_oci-v1.7.5.tar.gz",
)

load("@rules_oci//oci:dependencies.bzl", "rules_oci_dependencies")
rules_oci_dependencies()

load("@rules_oci//oci:repositories.bzl", "LATEST_CRANE_VERSION", "oci_register_toolchains")
oci_register_toolchains(
    name = "oci",
    crane_version = LATEST_CRANE_VERSION,
    # Uncommenting the zot toolchain will cause it to be used instead of crane for some tasks.
    # Note that it does not support docker-format images.
    # zot_version = LATEST_ZOT_VERSION,
)

load("@rules_oci//oci:pull.bzl", "oci_pull")
oci_pull(
    name = "distroless_cc",
    digest = "sha256:8aad707f96620ee89e27febef51b01c6ff244277a3560fcfcfbe68633ef09193",
    image = "gcr.io/distroless/cc",
    platforms = ["linux/amd64","linux/arm64"],
)

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
        "//:queng_components/db_system_manager/Cargo.toml",
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
        "//:queng_utils/db_utils/Cargo.toml",
        "//:queng_utils/proto_utils/Cargo.toml",
        "//:queng_utils/service_utils/Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")
crate_repositories()

