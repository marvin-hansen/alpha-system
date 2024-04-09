#!/usr/bin/env bash
set -eo pipefail

git tag "$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)" HEAD

git push --tags

export TAG="$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)"

echo TAG "$TAG"
