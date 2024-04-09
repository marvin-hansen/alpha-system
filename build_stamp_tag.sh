#!/usr/bin/env bash
set -eo pipefail

export TAG="$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)"
export BUILD_EMBED_LABEL="$TAG"
echo TAG "$TAG"
echo BUILD_EMBED_LABEL "$BUILD_EMBED_LABEL"