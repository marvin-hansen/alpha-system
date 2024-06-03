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
    integrity = "sha256-XT1YVJ6FHJTXBr1v3px2fV37/OCS3dQk3ul+XvfIIf8=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.42.0/rules_rust-v0.42.0.tar.gz"],
)

RUST_EDITION = "2021"
RUST_VERSION = "1.78.0"

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

# https://images.chainguard.dev/directory/image/static/versions
# Releases: https://edu.chainguard.dev/chainguard/chainguard-images/reference/static/tags_history/
load("@rules_oci//oci:pull.bzl", "oci_pull")
oci_pull(
    name = "distroless",
    digest = "sha256:8665c8a9fcdab0f8afc09533ee23287c7870de26064d464a10e3baa52f337734",
    image = "cgr.dev/chainguard/static",
    platforms = ["linux/amd64","linux/arm64"],
)

# https://images.chainguard.dev/directory/image/curl/versions
# https://hub.docker.com/r/chainguard/curl/tags?page=&page_size=&ordering=&name=latest
# Releases: https://edu.chainguard.dev/chainguard/chainguard-images/reference/static/tags_history/
load("@rules_oci//oci:pull.bzl", "oci_pull")
oci_pull(
    name = "curl_image",
    digest = "sha256:ee18838499f08683608f10de1571d1a2a162090e1d10faade140eff5269cb5fb",
    image = "cgr.dev/chainguard/curl",
    platforms = ["linux/amd64","linux/arm64"],
)

###############################################################################
# BuildBuddy Toolchain
# Releases: https://github.com/buildbuddy-io/buildbuddy-toolchain/
###############################################################################
http_archive(
    name = "io_buildbuddy_buildbuddy_toolchain",
    sha256 = "e899f235b36cb901b678bd6f55c1229df23fcbc7921ac7a3585d29bff2bf9cfd",
    strip_prefix = "buildbuddy-toolchain-fd351ca8f152d66fc97f9d98009e0ae000854e8f",
    urls = ["https://github.com/buildbuddy-io/buildbuddy-toolchain/archive/fd351ca8f152d66fc97f9d98009e0ae000854e8f.tar.gz"],
)

load("@io_buildbuddy_buildbuddy_toolchain//:deps.bzl", "buildbuddy_deps")
buildbuddy_deps()

load("@io_buildbuddy_buildbuddy_toolchain//:rules.bzl", "buildbuddy")
buildbuddy(name = "buildbuddy_toolchain")

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
    lockfile = "//:cargo-bazel-lock.json",
    packages = {
            "anyhow": crate.spec(
                          version = "1.0",
            ),

            "arc-swap": crate.spec(
                          version = "1.7",
            ),

            "binance-rs-async": crate.spec(
                         default_features=False,
                         features = ["rustls-tls"],
                         version = "1.3",
            ),

            "chrono": crate.spec(
                      features = ["serde"],
                      version = "0.4",
            ),

            "config-file": crate.spec(
                         version = "0.2",
            ),

            "ctrlc": crate.spec(
                          version = "3.4",
            ),

            "clickhouse-rs": crate.spec(
                             version = "1.1.0-alpha.1",
            ),

            "csv": crate.spec(
                         default_features=False,
                         version = "1.3",
            ),

            "encoding_rs": crate.spec(
                             version = "0.8",
            ),

            "futures": crate.spec(
                         version = "0.3",
            ),

            "hickory-resolver": crate.spec(
                         default_features=False,
                         features = ["tokio-runtime"],
                         version = "0.24",
            ),

            "klickhouse": crate.spec(
                         version = "0.13",
            ),

            "lru": crate.spec(
                         version = "0.12",
            ),

            "parquet": crate.spec(
                         version = "51.0",
            ),

            "prost": crate.spec(
                         version = "0.12",
            ),

            "prost-types": crate.spec(
                         default_features=False,
                         version = "0.12",
            ),

            "reqwest": crate.spec(
                         default_features=False,
                         features = ["rustls-tls", "json", "blocking", "gzip"],
                         version = "0.12",
            ),

            "rust_decimal": crate.spec(
                         version = "1.34",
            ),

            "rust_decimal_macros": crate.spec(
                       version = "1.34",
            ),

            "serde": crate.spec(
                      features = ["derive"],
                      version = "1.0",
            ),

            "serde_json": crate.spec(
                         version = "1",
            ),

            "scraper": crate.spec(
                      default_features=False,
                      version = "0.19",
            ),

            "tonic": crate.spec(
                      features = ["transport"],
                      version = "0.11",
            ),

            "tonic-build": crate.spec(
                      version = "0.11",
            ),

            "tonic-health": crate.spec(
                      default_features=False,
                      features = ["transport"],
                      version = "0.11",
            ),

            "tokio": crate.spec(
                     default_features=False,
                     features =  ["macros", "net", "rt-multi-thread", "signal"],
                     version = "1.38",
            ),

            "tokio-cron": crate.spec(
                     version = "0.1.3",
            ),


            "tokio-tungstenite": crate.spec(
                     default_features=False,
                     version = "0.23",
            ),

            "tokio-timer": crate.spec(
                     version = "0.2",
            ),

            "warp": crate.spec(
                         version = "0.3",
            ),
    },

)

load("@crate_index//:defs.bzl", "crate_repositories")
crate_repositories()
