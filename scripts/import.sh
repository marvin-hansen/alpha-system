# bin/sh
set -o errexit
set -o nounset
set -o pipefail

ENV=LOCAL bazel run //alias:pg_import_services

ENV=LOCAL bazel run //alias:pg_import_metadata
