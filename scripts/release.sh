# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Format all code
command command cargo fmt --all

# Compile everything in release mode
command bazel build  -c opt //...

# Run all tests
command bazel test  -c opt //...

# Build all docs and run doc tests
command bazel build  -c opt //:doc

# Build all container images in release mode
command bazel build -c opt //:image

# Pushes all tagged images to registry
command bazel run -c opt //:push