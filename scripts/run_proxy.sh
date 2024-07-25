# bin/sh
set -o errexit
set -o nounset
set -o pipefail

ENV=LOCAL bazel run //queng_services/aux/kaiko_proxy:bin
