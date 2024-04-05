#
# Copyright (c) 2021. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com
#

# bin/bash
set -o errexit
set -o nounset
set -o pipefail

# "---------------------------------------------------------"
# "-                                                       -"
# "-  Configures gcloud and terraform for automation       -"
# "-                                                       -"
# "---------------------------------------------------------"

command -v gcloud >/dev/null 2>&1 || {
  echo >&2 "GCloud required but it's not installed. Run: make setup "
  exit 1
}

echo ""
echo "==============================="
echo " Configure gcloud & Kubectl: "
echo "==============================="
echo ""

PROJECT=$(gcloud config get-value core/project)
if [[ -z "${PROJECT}" ]]; then
  echo "gcloud cli must be configured with a default project." 1>&2
  echo "Initialize and configure gcloud now"
  command gcloud init
fi
echo "* Get the default gcp project: Check"

REGION=$(gcloud config get-value compute/region)
if [ -z "${REGION}" ]; then
  echo "gcloud cli must be configured with a default zone." 1>&2
  echo "run 'gcloud config set compute/region REGION'." 1>&2
  echo "replace 'ZONE' with the region name like us-west1." 1>&2
  exit 1
fi
echo "* Get the default gcp region: Check"

ZONE=$(gcloud config get-value compute/zone)
if [ -z "${ZONE}" ]; then
  echo "gcloud cli must be configured with a default zone." 1>&2
  echo "run 'gcloud config set compute/zone ZONE'." 1>&2
  echo "replace 'ZONE' with the zone name like us-west1-a." 1>&2
  exit 1
fi
echo "* Get the default gcp zone: Check"

command gcloud services enable compute.googleapis.com \
  container.googleapis.com \
  cloudbuild.googleapis.com \
  cloudresourcemanager.googleapis.com
echo "* Enable GCloud APIs: Check"

# Check if logged in to gcloud; otherwise  authenticate
TOKEN=$(gcloud auth application-default print-access-token)
if [ -z "${ZONE}" ]; then
  echo "Authentication for terraform"
  command gcloud auth application-default login
fi
echo "* Authentication: Check"

command gcloud config get-value project >/dev/null 2>&1 || {
  command gcloud container clusters get-credentials quantum --zone="us-east4"
}

PROJECT=$(gcloud config get-value project)
CONTEXT=$(kubectl config get-contexts -o=name | grep "gke_$PROJECT*")
REPO=gcr.io/$PROJECT
echo "* GCloud project configured: Check"

command kubectl config use-context $CONTEXT
echo "* kubectl configured: Check"

command helm repo add stable https://charts.helm.sh/stable
command helm repo add timescale 'https://charts.timescale.com'
command helm repo update
echo "* Helm configured: Check"

echo ""
echo "==============================="
echo "All configuration completed."
echo "==============================="
echo "* GCloud: Ready"
echo "* Kubectl: Ready"
echo "* Helm: Ready"
echo "==============================="
echo ""

echo ""
echo "==============================="
echo " Cloud project settings:      "
echo "==============================="
echo ""
echo "Project: $PROJECT"
echo "Cluster: $CONTEXT"
echo "Context: $CONTEXT"
echo "Repo: $REPO"
echo ""

echo ""
echo "To deploy a new cluster, "
echo "please run the following command: "
echo "==============================="
echo "* make create-cluster          "
echo "==============================="
echo ""
