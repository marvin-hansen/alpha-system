# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# When adding new bazel commands, also update buildbuddy.yaml with the same command for CI testing

command bazel build //...

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
echo "Setup Containers"
echo "=============="
command bazel test //... --test_tag_filters=env_container_setup

echo "====================="
echo "Run integration tests"
echo "====================="

echo ""

echo "====================="
echo "Test: API Proxy integration"
echo "====================="
command bazel test //... --test_tag_filters=clickhouse_utils_integration_tests

echo "====================="
echo "Test: Clickhouse Integration"
echo "====================="
command bazel test //... --test_tag_filters=clickhouse_utils_integration_tests

echo "====================="
echo "Build container images"
echo "====================="
command bazel build //:push