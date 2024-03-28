# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Compile everything in release mode
command  bazel build -c opt //...

# Run all tests
command  bazel test -c opt //...

# Build all container images in release mode
command  bazel build -c opt //:image

# Push all container images to the registry
# /bin/bash scripts/bazel/push.sh
