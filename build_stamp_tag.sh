#!/usr/bin/env bash
set -e

export TAG="$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)"
echo TAG "$TAG"
