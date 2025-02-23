#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Compile everything again using the updated dependencies
command bazel build //...
