#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Run with Bazel
ENV=LOCAL bazel run  -c opt //alias/service:kaiko_proxy

# Docker network is about 20x slower than localhost when accessing the proxy locally.
# Find image tag and URL in queng_specs/container_specs/container_specs_kaiko/src/lib.rs
#command docker run --name apiproxy-7777 -p 7777:7777 -e ENV=LOCAL -d asia-northeast1-docker.pkg.dev/future-309012/image-repo/kaiko_proxy:96fd9937-1729768888
