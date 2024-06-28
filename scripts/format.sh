# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Bazel file formatting (Installed via homebrew)
# https://github.com/bazelbuild/buildtools
buildifier -r queng_* MODULE.bazel BUILD.bazel WORKSPACE.bzlmod

# Rust code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all