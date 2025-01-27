# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Builds all images
command bazel build //:push --test_env=ENV=LOCAL
