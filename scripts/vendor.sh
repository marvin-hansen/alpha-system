# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Updates base image to latest version
command bazel run @rules_apko//apko lock images/base_image/apko.yaml

# Updates all vendored crates
command bazel run //thirdparty:crates_vendor