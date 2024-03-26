# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel run //queng_services/cmdb:push
command bazel run //queng_services/ims/data/binance_data:push
command bazel run //queng_services/smdb:push
command bazel run //queng_services/symdb:push