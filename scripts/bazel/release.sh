# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command command cargo fmt --all

# Compile everything in release mode
bazel build  --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             -c opt //...

# Run all tests
bazel test   --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             -c opt //...

# Build all docs and run doc tests
bazel build  --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             -c opt //:doc

# Build all container images in release mode
bazel build  --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             -c opt //:image

# Publish all Docker images to the registry
command bazel run -c opt //queng_services/cmdb:push
command bazel run -c opt //queng_services/ims/data/binance_data:push
command bazel run -c opt //queng_services/smdb:push
command bazel run -c opt //queng_services/symdb:push
