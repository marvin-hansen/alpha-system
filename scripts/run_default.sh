# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel run //queng_cli/symbol_mapping:bin
