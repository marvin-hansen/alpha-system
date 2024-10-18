# bin/sh
set -o errexit
set -o nounset
set -o pipefail

ENV=LOCAL bazel run //alias:kaiko_proxy
