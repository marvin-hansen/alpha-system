# bin/sh
set -o errexit
set -o nounset
set -o pipefail


MIGRATION_DATA="migrations"  cargo fix --allow-dirty

MIGRATION_DATA="migrations"  cargo clippy --fix --allow-dirty