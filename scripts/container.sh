# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Builds all images
command bazel build -c opt  //:image
# Generates tags for all images
command bazel build //:build_stamp_tag
# Pushes all tagged images to registry
command baze run -c opt //:push --stamp --embed_label="$TAG"