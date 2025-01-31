#!/usr/bin/env bash
#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

set -o errexit
set -o nounset
set -o pipefail

# Manual registry auth setup via config.json file
# https://www.flatcar.org/docs/latest/container-runtimes/registry-authentication/#manual-registry-auth-setup

# For GPC authentication, the registry the URL *MUST* have the following format (including trailing slash):
# region.pkg.dev/project/repo/
# otherwise you will see unauthenticated errors at image pulls.

# For Docker authentication, the $DOCKER_AUTH token is user:password or user:token

mkdir -p ~/.docker

GCP_AUTH=$(echo -n "_json_key:$CONTAINER_REGISTRY_SERVICE_ACCOUNT_KEY_JSON" | base64 -w 0)

DOCKER_AUTH=$(echo -n "$DOCKER_HUB_TOKEN" | base64)

cat > ~/.docker/config.json <<EOF
{
  "auths": {
      "asia-northeast1-docker.pkg.dev/future-309012/image-repo/": {
        "auth": "$GCP_AUTH"
      },
     "https://index.docker.io/v1/": {
        "auth": "$DOCKER_AUTH"
      }
   }
}
EOF
