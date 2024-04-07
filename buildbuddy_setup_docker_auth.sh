# bin/sh
set -o errexit
set -o nounset
set -o pipefail
set -e

mkdir -p ~/.docker
AUTH=$(echo -n "_json_key:$SERVICE_ACCOUNT_KEY_JSON" | base64 -w 0)
cat > ~/.docker/config.json <<EOF
{"auths": {"gcr.io": {"auth": "$AUTH"}}}
EOF
