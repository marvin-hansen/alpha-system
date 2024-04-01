# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command command cargo fmt --all

# Compile everything
bazel build  --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             //...

# Build all docs
bazel build  --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             //:doc

# Run all tests & upload results to BES
bazel test --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             //...

## Publish all Docker images to the registry
#command bazel run -c opt //queng_services/cmdb:push
#command bazel run -c opt //queng_services/ims/data/binance_data:push
#command bazel run -c opt //queng_services/smdb:push
#command bazel run -c opt //queng_services/symdb:push

# Push all remaining commits to remote
command git push