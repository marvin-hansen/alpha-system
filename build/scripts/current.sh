#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Load current Bazel target
current=$(cat current.txt)

#echo "$current"
command bazel build "$current" --test_env=ENV=LOCAL