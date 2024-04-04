# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Syncs Cargo dependencies to Bazel index
CARGO_BAZEL_ISOLATED=false CARGO_BAZEL_REPIN=1 bazel --host_jvm_args=-Xmx2g sync --only=crate_index

# Compile everything
bazel --host_jvm_args=-Xmx2g build //... --jobs=50

# Run all tests & upload results to BES
bazel --host_jvm_args=-Xmx2g \
             test \
             --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             //... \
             --jobs=50
