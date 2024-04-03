# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command kubectl create secret docker-registry artifact-registry \
  --namespace=quant-engine \
  --docker-server=https://asia-northeast1-docker.pkg.dev \
  --docker-email=container-puller@future-309012.iam.gserviceaccount.com \
  --docker-username=_json_key \
  --docker-password="$(cat serviceaccount.json)"

# Verify the creation of Kubernetes Secret:
command kubectl get secrets | grep artifact-registry