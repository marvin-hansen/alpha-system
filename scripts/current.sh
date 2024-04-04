# bin/sh
set -o errexit
set -o nounset
set -o pipefail

current=$(cat current.txt)

#echo "$current"
bazel --host_jvm_args=-Xmx2g build "$current" --jobs=50