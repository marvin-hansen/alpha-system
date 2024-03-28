# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Syncs Cargo dependencies to Bazel index
CARGO_BAZEL_ISOLATED=false CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index

# Rebuilds the entire workspace
command bazel build //:build
