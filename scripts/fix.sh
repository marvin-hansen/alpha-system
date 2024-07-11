# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cargo fix -k --lib --allow-dirty

command cargo clippy --fix --lib --allow-dirty