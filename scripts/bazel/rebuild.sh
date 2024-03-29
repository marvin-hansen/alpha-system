# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Syncs Cargo dependencies to Bazel index
CARGO_BAZEL_ISOLATED=false CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index

# Rebuilds the entire workspace
bazel build  --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             //...
