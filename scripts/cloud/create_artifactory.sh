# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# https://heeten.github.io/hello-monorepo-bazel/07_cloud_run.html
gcloud artifacts repositories create image-repo --repository-format=docker \
--location=asia-northeast1 --description="OCI IMAGE repository" \
  --project=future-309012

gcloud artifacts repositories list --project=future-309012

gcloud auth configure-docker asia-northeast1-docker.pkg.dev