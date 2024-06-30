#!/usr/bin/env bash
set -e

mkdir -p ~/.docker

GCP_AUTH=$(echo -n "_json_key:$CONTAINER_REGISTRY_SERVICE_ACCOUNT_KEY_JSON" | base64 -w 0)

DOCKER_AUTH=$(echo -n "$DOCKER_HUB_TOKEN" | base64)

cat > ~/.docker/config.json <<EOF
{"auths":
  {"asia-northeast1-docker.pkg.dev": {"auth": "$GCP_AUTH"}},
  {"https://index.docker.io/v1/": {"auth": "$DOCKER_AUTH"}},
}
EOF
