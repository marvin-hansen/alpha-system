# bin/sh
set -o errexit
set -o nounset
set -o pipefail

echo "=============="
echo "Run doc tests"
echo "=============="
command bazel build //... --build_tag_filters=doc-test
command bazel test //... --test_tag_filters=doc-test

echo "=============="
echo "Run unit tests"
echo "=============="
command bazel build //... --build_tag_filters=unit
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
