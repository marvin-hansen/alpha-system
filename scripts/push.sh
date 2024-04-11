# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Format all source code.
command command cargo fmt --all

# Kick off remote build and test in the background to speed up CI
command bazel build -c opt //... --config=remote &
command bazel test  -c opt //... --config=remote &

# Compile everything
command bazel build //...

# Build all docs
command bazel build //:doc

# Run all tests & upload results to BES
command bazel test --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             //...

# Push all new and remaining commits to remote to trigger CI
command git push
