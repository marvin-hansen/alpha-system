# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Format all source code.
command command cargo fmt --all

# Kick off remote build and test with bazel to ensure hot cache for CI
command bazel build -c opt //... --config=remote

command bazel test  -c opt //... --test_tag_filters=unit --config=remote

# Push all new and remaining commits to remote to trigger CI
command git push
