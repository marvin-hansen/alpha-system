# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Run all unit tests first
echo "=============="
echo "Run unit tests"
echo "=============="
command bazel test //... --test_tag_filters=unit

# Run all unit tests first
echo "=============="
echo "Setup Test Env"
echo "=============="
command bazel test //... --test_tag_filters=env_setup

