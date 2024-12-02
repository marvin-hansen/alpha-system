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
# local testing must be in sequential order b/c there is just one DB
command bazel test //... --test_tag_filters=smdb_acceptance_test --test_env=ENV=LOCAL
command bazel test //... --test_tag_filters=cmdb_acceptance_test --test_env=ENV=LOCAL
command bazel test //... --test_tag_filters=mddb_acceptance_test --test_env=ENV=LOCAL
command bazel test //... --test_tag_filters=imdb_acceptance_test --test_env=ENV=LOCAL

echo ""
echo "====================="
echo "Run IMS acceptance tests"
echo "====================="
command bazel test //... --test_tag_filters=binance_data_acceptance_test --test_env=ENV=LOCAL


echo ""
echo "====================="
echo "Build container images"
echo "====================="

command bazel build //:push --test_env=ENV=LOCAL


# Check if a docker container with the name postgres-5432 is already running
if [ "$(docker ps --filter "name=postgres-5432" --filter "status=running" -q)" ]; then
 exit 0
fi

# Start the container if it is not running
echo "Starting a new container postgres-5432!"
docker run --name postgres-5432 -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres:17-alpine3.20

echo ""
echo "====================="
echo "All Tests Passed"
echo "====================="
echo ""