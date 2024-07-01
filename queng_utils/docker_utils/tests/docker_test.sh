#!/usr/bin/env bash
set -o errexit
set -o nounset
set -o pipefail

docker pull --platform linux/amd64 hansenmarvin/api_proxy:b422ae3
