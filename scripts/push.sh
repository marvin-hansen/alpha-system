# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Format all source code.
command command cargo fmt --all

# Compile everything
command bazel build //... --jobs=50

# Build all docs
command bazel build //:doc --jobs=50

# Run all tests & upload results to BES
command bazel test --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             //... \
             --jobs=50

# Push all new and remaining commits to remote to trigger CI
command git push
