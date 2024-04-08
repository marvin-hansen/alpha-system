# bin/sh
set -o errexit
set -o nounset
set -o pipefail

echo CURRENT_COMMIT "$(git rev-parse --short HEAD)"
