#!/usr/bin/env bash
set -e

mkdir -p ~/.docker

AUTH=$(echo -n "_json_key:$CONTAINER_REGISTRY_SERVICE_ACCOUNT_KEY_JSON" | base64 -w 0)

cat > ~/.docker/config.json <<EOF
{"auths": {"asia-northeast1-docker.pkg.dev": {"auth": "$AUTH"}}}
EOF
