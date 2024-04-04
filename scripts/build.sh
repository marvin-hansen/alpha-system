# bin/sh
set -o errexit
set -o nounset
set -o pipefail


bazel --host_jvm_args=-Xmx2g build //... --jobs=50