# bin/sh
set -o errexit
set -o nounset
set -o pipefail

ENV=LOCAL bazel run //alias/service:kaiko_proxy
