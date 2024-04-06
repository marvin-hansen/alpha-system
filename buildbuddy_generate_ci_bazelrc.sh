# bin/sh
set -o errexit
set -o nounset
set -o pipefail
set -e

# Change to the WORKSPACE directory
cd "$BUILD_WORKSPACE_DIRECTORY"

# Run a command to request image pull credentials:
REGISTRY_PASSWORD=$(gcloud auth print-access-token)

# Write the credentials to ci.bazelrc in the workspace root directory:
echo >ci.bazelrc "
build --remote_exec_header=x-buildbuddy-platform.container-registry-username=_dcgcloud_token
build --remote_exec_header=x-buildbuddy-platform.container-registry-password=${REGISTRY_PASSWORD}
"