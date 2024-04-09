#!/usr/bin/env bash
set -eo pipefail

export TAG="$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)"
export BUILD_VERSION="$TAG"
echo TAG "$TAG"
echo BUILD_VERSION "$BUILD_VERSION"