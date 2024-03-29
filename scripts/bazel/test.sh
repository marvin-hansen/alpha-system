# bin/sh
set -o errexit
set -o nounset
set -o pipefail


bazel test  --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             //...