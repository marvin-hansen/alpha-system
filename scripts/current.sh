# bin/sh
set -o errexit
set -o nounset
set -o pipefail

current=$(cat current.txt)

#echo "$current"
command bazel build "$current" --jobs=50