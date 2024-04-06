# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command command cargo fmt --all

# Compile everything
command bazel build //... --jobs=50

# Build all docs
command bazel build //:doc --jobs=50

# Run all tests & upload results to BES
command bazel test --bes_results_url=https://app.buildbuddy.io/invocation/ \
             --bes_backend=grpcs://remote.buildbuddy.io \
             //... \
             --jobs=50

# Run a command to generate temporary image pull credentials that expire after 30 minutes:
REGISTRY_PASSWORD=$(gcloud auth print-access-token --impersonate-service-account=105782787584889908240 --lifetime=1800 --quiet)

# Write the credentials to ci.bazelrc in the workspace root directory:
echo >ci.bazelrc "
build --remote_exec_header=x-buildbuddy-platform.container-registry-username=_dcgcloud_token
build --remote_exec_header=x-buildbuddy-platform.container-registry-password=${REGISTRY_PASSWORD}
"

# Commit the file so that CI can read the temporary credentials
command git commit -m "Added ci.bazelrc with temporary image pull credentials"

# Push all new and remaining commits to remote to trigger CI
command git push