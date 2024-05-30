# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel run -c opt //queng_services/aux/file_server:bin
