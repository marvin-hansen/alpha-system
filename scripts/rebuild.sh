# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Syncs Cargo dependencies to Bazel index
CARGO_BAZEL_REPIN=true bazel build //...

# Compile everything
command bazel build //... --jobs=50

# Run all tests & upload results to BES
command bazel \
             test \
             --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             //... \
             --jobs=50
