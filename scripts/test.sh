# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Run all unit tests first
echo "=============="
echo "Run unit tests"
echo "=============="
command bazel test //... --test_tag_filters=unit

echo "=============="
echo "Setup Test Env"
echo "=============="
command bazel test //... --test_tag_filters=env_setup

echo "====================="
echo "Run integration tests"
echo "====================="
# First check if the utils are working as expected before running the integration tests
command bazel test //... --test_tag_filters=clickhouse_utils
# Then run the integration tests for the DB components
