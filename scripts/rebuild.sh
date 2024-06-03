# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Syncs Cargo dependencies to Bazel index
CARGO_BAZEL_REPIN=true bazel build //...

# Compile everything again using the updated dependencies
command bazel build //...

# Run all tests again to double check everything is okay.
command bazel test //... --test_tag_filters=unit