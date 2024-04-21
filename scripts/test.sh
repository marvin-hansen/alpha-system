# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel test //...

#command bazel \
#             test \
#             --bes_results_url=https://app.buildbuddy.io/invocation/ \
#             --bes_backend=grpcs://remote.buildbuddy.io \
#             //... \
#             --jobs=50
