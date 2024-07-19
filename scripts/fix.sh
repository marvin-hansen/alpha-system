# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cargo fix --allow-dirty

command cargo clippy --fix --allow-dirty