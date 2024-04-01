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

# Run all tests & upload results to BES
bazel test --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             --nolegacy_important_outputs \
             --noslim_profile \
             --experimental_remote_cache_compression \
             --experimental_profile_include_target_label \
             --experimental_profile_include_primary_output \
             //...

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

# Build and publish all Docker images to the registry

command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=cmdb"  --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/cmdb_arm64:latest -f Dockerfile .
command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=dbgw"  --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/dbgw_arm64:latest -f Dockerfile .
command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=smdb"  --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/smdb_arm64:latest -f Dockerfile .
command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=symdb" --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/symdb_arm64:latest -f Dockerfile .

command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/cmdb_arm64:latest
command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/dbgw_arm64:latest
command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/smdb_arm64:latest
command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/symdb_arm64:latest