#!/usr/bin/env bash
set -eo pipefail

export TAG="$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)"export BUILD_EMBED_LABEL="$TAG"
git tag $TAG HEAD
git push --tags

echo TAG "$TAG"
