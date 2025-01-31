#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

REPO_NAME="OCI IMAGE repository"

echo ""
echo "==============================="
echo " Create new artifactory: " "$REPO_NAME"
echo "==============================="
echo ""

# Test for GKE default project and prompt when missing.
PROJECT=$(gcloud config get-value core/project)
if [[ -z "${PROJECT}" ]]; then
  echo "gcloud cli must be configured with a default project." 1>&2
  echo "Initialize and configure gcloud now"
  command gcloud init
fi

# Test for GKE default zone and prompt when missing.
ZONE=$(gcloud config get-value compute/zone)
if [ -z "${ZONE}" ]; then
  echo "gcloud cli must be configured with a default zone." 1>&2
  echo "run 'gcloud config set compute/zone ZONE'." 1>&2
  echo "replace 'ZONE' with the zone name like us-west1-a." 1>&2
  exit 1
fi

# https://heeten.github.io/hello-monorepo-bazel/07_cloud_run.html
gcloud artifacts repositories create image-repo --repository-format=docker \
--location="${ZONE}" --description="$REPO_NAME" \
  --project=future-309012

gcloud artifacts repositories list --project=future-309012

gcloud auth configure-docker asia-northeast1-docker.pkg.dev