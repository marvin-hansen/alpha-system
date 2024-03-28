workspace(name = "queng")

###############################################################################
# R U L E S  S K Y L I B
###############################################################################
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
rust_version = "1.77.0"

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

###############################################################################
# R U L E S  O C I  I M A G E
###############################################################################
# https://github.com/bazel-contrib/rules_oci
# https://github.com/bazel-contrib/rules_oci/releases
http_archive(
    name = "rules_oci",
    sha256 = "4a276e9566c03491649eef63f27c2816cc222f41ccdebd97d2c5159e84917c3b",
    strip_prefix = "rules_oci-1.7.4",
    url = "https://github.com/bazel-contrib/rules_oci/releases/download/v1.7.4/rules_oci-v1.7.4.tar.gz",
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

################################################################################
# Kubernetes rules
# https://github.com/bazelbuild/rules_k8s/releases/
################################################################################
# https://github.com/bazelbuild/rules_docker/#setup
http_archive(
    name = "io_bazel_rules_k8s",
    sha256 = "ce5b9bc0926681e2e7f2147b49096f143e6cbc783e71bc1d4f36ca76b00e6f4a",
    strip_prefix = "rules_k8s-0.7",
    urls = ["https://github.com/bazelbuild/rules_k8s/archive/refs/tags/v0.7.tar.gz"],
)

load("@io_bazel_rules_k8s//k8s:k8s.bzl", "k8s_repositories")

k8s_repositories()

load("@io_bazel_rules_k8s//k8s:k8s_go_deps.bzl", k8s_go_deps = "deps")

k8s_go_deps()

# Set up some default attributes when the K8s rule "k8s_object" is called later,
# This also exposes a Bazel rule called "k8s_deploy"
# See https://github.com/bazelbuild/rules_k8s#k8s_defaults
load("@io_bazel_rules_k8s//k8s:k8s.bzl", "k8s_defaults")

## ==== Deploy Defaults
## import: load("@k8s_deploy//:defaults.bzl", "k8s_deploy")
k8s_defaults(
    name = "k8s_deploy",
    # kubectl config current-context
    cluster = "gke_future-309012_asia-northeast1-c_quantum",
    context = "gke_future-309012_asia-northeast1-c_quantum",
    image_chroot = "gcr.io/future-309012",
    kind = "deployment",
    namespace = "default",
)

## ==== Dev Deploy Defaults
## import: load("@k8s_deploy_dev//:defaults.bzl", "k8s_deploy_dev")
k8s_defaults(
    name = "k8s_deploy_dev",
    cluster = "gke_future-309012_asia-northeast1-c_quantum",
    context = "gke_future-309012_asia-northeast1-c_quantum",
    image_chroot = "gcr.io/future-309012",
    kind = "deployment",
    namespace = "default",
)

## ==== Test Deploy Defaults
## import: load("@k8s_deploy_test//:defaults.bzl", "k8s_deploy_test")

k8s_defaults(
    name = "k8s_deploy_test",
    cluster = "gke_future-309012_asia-northeast1-c_quantum",
    context = "gke_future-309012_asia-northeast1-c_quantum",
    image_chroot = "gcr.io/future-309012",
    kind = "deployment",
    namespace = "default",
)

## ==== Prod Deploy Defaults
## import: load("@k8s_deploy_prod//:defaults.bzl", "k8s_deploy_prod")
k8s_defaults(
    name = "k8s_deploy_prod",
    cluster = "gke_future-309012_asia-northeast1-c_quantum",
    context = "gke_future-309012_asia-northeast1-c_quantum",
    image_chroot = "gcr.io/future-309012",
    kind = "deployment",
    namespace = "default",
)

# Set up some default attributes when the K8s rule "k8s_object" is called later
# See https://github.com/bazelbuild/rules_k8s#k8s_defaults
k8s_defaults(
    name = "k8s_service",
    kind = "service",
    namespace = "default",
)
