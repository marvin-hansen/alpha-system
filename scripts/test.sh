# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel test //... --test_tag_filters=unit

command bazel test //... --test_tag_filters=components

command bazel test //... --test_tag_filters=sbe

command bazel test //... --test_tag_filters=specs

