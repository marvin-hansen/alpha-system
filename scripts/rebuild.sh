# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Compile everything again using the updated dependencies
command bazel build //...
