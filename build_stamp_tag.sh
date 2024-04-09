#!/usr/bin/env bash
set -eo pipefail

echo STABLE_GIT_COMMIT "$(date +"%Y.%m.%d")"."$(git rev-parse --short HEAD)"
