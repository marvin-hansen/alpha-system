# bin/sh
set -o errexit
set -o nounset
set -o pipefail

ENV=LOCAL bazel run //queng_import/postgres/pg_import_services