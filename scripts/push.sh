# bin/sh
set -o errexit
set -o nounset
set -o pipefail

echo "=============="
echo "Lint targets "
echo "=============="
command cargo clippy --all-targets

echo "=============="
echo "Format targets "
echo "=============="
# Bazel file formatting (Installed via homebrew)
# https://github.com/bazelbuild/buildtools
command buildifier -r MODULE.bazel BUILD.bazel thirdparty/BUILD.bazel
command buildifier -r build images queng_*

# Rust code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all

# Check for uncommited changes before building and testing.
# It is possible that either image update or fie format changed some files,
# so it is important to check for uncommited changes before continuing.
if [[ $(git status --porcelain | wc -l) -gt 0 ]];
then
  #
  echo "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  echo "Uncommited changes found; commit first, then run script again"
  echo "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  # Full stop
  exit 1
fi

echo "=============="
echo "Build targets "
echo "=============="
command bazel build //...
command bazel build //... --build_tag_filters=doc-test --test_env=ENV=LOCAL

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
echo "Run IMS acceptance tests"
echo "====================="
# local testing must be in sequential order b/c there is just one DB container available
command bazel test //... --test_tag_filters=ims-acceptance_test --test_env=ENV=LOCAL

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
echo "Build container images"
echo "====================="
command bazel build //:push --test_env=ENV=LOCAL

# Double check again for uncommited changes before pushing to git remote
if [[ $(git status --porcelain | wc -l) -gt 0 ]];
then
  echo "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  echo "Uncommited changes found; commit first, then run script again"
  echo "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"

  exit 1
else
  # echo NOT CHANGED locally
  # Push all new and remaining commits to remote to trigger CI
  echo "====================="
  echo "Push to git remote"
  echo "====================="
  command git push
fi

echo "Completed"
exit 0