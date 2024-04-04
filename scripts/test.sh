# bin/sh
set -o errexit
set -o nounset
set -o pipefail


bazel test --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             //...