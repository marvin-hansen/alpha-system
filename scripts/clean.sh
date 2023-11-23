# bin/bash
set -o errexit
set -o nounset
set -o pipefail

command cargo clean

command rm -rf sbe/bindings
command rm -rf sbe/car.sbeir
