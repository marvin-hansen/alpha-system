#!/usr/bin/env bash
set -eo pipefail

export TAG="$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)"
echo TAG "$TAG"
