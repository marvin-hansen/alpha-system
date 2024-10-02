# bin/sh
set -o errexit
set -o nounset
set -o pipefail

echo "=============="
echo "Update images "
echo "=============="
command bazel run  @rules_apko//apko lock images/base_image/apko.yaml

echo "=============="
echo "Format targets "
echo "=============="
# Bazel file formatting (Installed via homebrew)
# https://github.com/bazelbuild/buildtools
command buildifier -r MODULE.bazel BUILD.bazel WORKSPACE.bzlmod thirdparty/BUILD.bazel
command buildifier -r build images queng_*

# Rust code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all

echo "=============="
echo "Build targets "
echo "=============="
command bazel build //...
command bazel build //... --build_tag_filters=doc-test

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
echo "Build container images"
echo "====================="
command bazel build //:push

if [[ $(git status --porcelain | wc -l) -gt 0 ]];
then
  echo "Uncommited changes found; commit first, then run script again"
  exit 1
else
  # echo NOT CHANGED locally
  # Push all new and remaining commits to remote to trigger CI
  echo "Push to git remote"
  command git push
fi

echo "Completed"
