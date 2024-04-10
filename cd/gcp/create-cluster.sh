#
# Copyright (c) 2021. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com
#

# bin/bash
set -o errexit
set -o nounset
set -o pipefail
# cluster name must be all lowercase
CLUSTER_NAME="quantum"
echo ""
echo "==============================="
echo " Create new cluster: " $CLUSTER_NAME
echo "==============================="
echo ""

# Test for GKE default zone and prompt when missing.
ZONE=$(gcloud config get-value compute/zone)
if [ -z "${ZONE}" ]; then
  echo "gcloud cli must be configured with a default zone." 1>&2
  echo "run 'gcloud config set compute/zone ZONE'." 1>&2
  echo "replace 'ZONE' with the zone name like us-west1-a." 1>&2
  exit 1
fi

#  Creates & configures a zonal cluster
# https://cloud.google.com/sdk/gcloud/reference/container/clusters/create#--enable-ip-alias
command gcloud container clusters create $CLUSTER_NAME \
  --zone $ZONE \
  --node-locations $ZONE \
  --machine-type="n2d-standard-2" \
  --enable-autoscaling --min-nodes 2 --max-nodes 5 --zone $ZONE \
  --enable-autoprovisioning --min-cpu 6 --min-memory 12 --max-cpu 12 --max-memory 48 --zone $ZONE \
  --max-unavailable-upgrade=2 \
  --max-nodes-per-pool=100 \
  --enable-vertical-pod-autoscaling \
  --enable-autoupgrade \
  --enable-autorepair \
  --enable-shielded-nodes \
  --shielded-secure-boot \
  --release-channel=regular \
  --enable-ip-alias \
  --image-type=COS_CONTAINERD \
  --disk-size=30GB \
  --disk-type=pd-ssd \
  --addons NodeLocalDNS

# Using node auto-provisioning
# https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning

# List of all available and valid image  types
# gcloud container get-server-config

# List all available machine types for zone
# gcloud compute machine-types list | grep 'us-east-*' | grep 'n2d-*'

# Choosing a minimum CPU platform
# https://cloud.google.com/kubernetes-engine/docs/how-to/min-cpu-platform

# Setting up NodeLocal DNSCache
#https://cloud.google.com/kubernetes-engine/docs/how-to/nodelocal-dns-cache

# Configuring vertical Pod autoscaling
# https://cloud.google.com/kubernetes-engine/docs/how-to/vertical-pod-autoscaling
