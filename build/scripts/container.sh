#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Builds all images
command bazel build //:push --test_env=ENV=LOCAL
