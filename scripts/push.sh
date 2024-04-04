# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command command cargo fmt --all

# Compile everything
bazel --host_jvm_args=-Xmx2g build //... --jobs=50

# Build all docs
bazel --host_jvm_args=-Xmx2g build //:doc --jobs=50

# Run all tests & upload results to BES
bazel --host_jvm_args=-Xmx2g test --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             //... \
             --jobs=50

# Push all remaining commits to remote
command git push