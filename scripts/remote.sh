# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel build -c opt //...  --config=remote

command bazel test -c opt //... --test_tag_filters=unit --config=remote