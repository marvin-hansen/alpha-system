# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel run //thirdparty:crates_vendor