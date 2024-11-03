# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# When adding new bazel commands, also update buildbuddy.yaml with the same command for CI testing

echo "=============="
echo "Build targets "
echo "=============="

command bazel build //... --test_env=ENV=LOCAL
command bazel build //... --build_tag_filters=doc-test,unit-test,integration_test,acceptance_test --test_env=ENV=LOCAL

echo ""
echo "=============="
echo "Run doc tests"
echo "=============="

command bazel test //... --test_tag_filters=doc-test --test_env=ENV=LOCAL

echo ""
echo "=============="
echo "Run unit tests"
echo "=============="

command bazel test //... --test_tag_filters=unit-test --test_env=ENV=LOCAL

echo ""
echo "====================="
echo "Run integration tests"
echo "====================="

command bazel test //... --test_tag_filters=integration_test --test_env=ENV=LOCAL

echo ""
echo "====================="
echo "Run acceptance tests"
echo "====================="

command bazel test //... --test_tag_filters=acceptance_test --test_env=ENV=LOCAL

echo ""
echo "====================="
echo "Build container images"
echo "====================="

command bazel build //:push --test_env=ENV=LOCAL