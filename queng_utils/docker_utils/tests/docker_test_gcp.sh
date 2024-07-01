#!/usr/bin/env bash
set -o errexit
set -o nounset
set -o pipefail

docker pull --platform linux/amd64 asia-northeast1-docker.pkg.dev/future-309012/image-repo/api_proxy:b422ae3

