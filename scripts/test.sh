# bin/sh
set -o errexit
set -o nounset
set -o pipefail


bazel --host_jvm_args=-Xmx2g \
             test \
             --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             //... \
             --jobs=50