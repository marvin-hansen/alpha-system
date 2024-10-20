# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Run with Bazel
# ENV=LOCAL bazel run  -c opt //alias:kaiko_proxy

# Find image tag and URL in queng_specs/container_specs/container_specs_kaiko/src/lib.rs
command docker run --name apiproxy-7777 -p 7777:7777 -e ENV=LOCAL -d index.docker.io/hansenmarvin/api_proxy:280562f
