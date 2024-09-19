#
# Copyright (c) 2021. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com
#

# bin/bash
set -o errexit
set -o nounset
set -o pipefail
# cluster name must be all lowercase
CLUSTER_NAME="quantum"
CLUSTER_ZONE="us-east4-a"
MACHINE_TYPE="c4a-standard-1"

echo ""
echo "==============================="
echo " Create new cluster: " $CLUSTER_NAME
echo "==============================="
echo ""

#  Creates & configures a zonal cluster
command gcloud container clusters create $CLUSTER_NAME \
  --zone $CLUSTER_ZONE \
  --node-locations $CLUSTER_ZONE \
  --machine-type=$MACHINE_TYPE \
  --enable-autoscaling --min-nodes 2 --max-nodes 5 --zone $CLUSTER_ZONE \
  --enable-autoprovisioning --min-cpu 4 --max-cpu 12 --min-memory 12 --max-memory 24 --zone $CLUSTER_ZONE \
  --max-unavailable-upgrade=2 \
  --max-nodes-per-pool=100 \
  --max-pods-per-node=100 \
  --enable-vertical-pod-autoscaling \
  --enable-autoupgrade \
  --enable-autorepair \
  --enable-shielded-nodes \
  --shielded-secure-boot \
  --shielded-integrity-monitoring \
  --release-channel=regular \
  --image-type=COS_CONTAINERD \
  --disk-size=50GB \
  --addons NodeLocalDNS \
  --no-enable-ip-alias \
  --no-autoprovisioning-enable-insecure-kubelet-readonly-port

# Configure maximum Pods per node
# https://cloud.google.com/kubernetes-engine/docs/how-to/flexible-pod-cidr

# Disable the kubelet read-only port
# https://cloud.google.com/kubernetes-engine/docs/how-to/disable-kubelet-readonly-port


PROJECT=$(gcloud config get-value core/project)
REGION=$(gcloud config get-value compute/region)
ZONE=$(gcloud config get-value compute/zone)

echo ""
echo "==============================="
echo " Cluster created:      "
echo "==============================="
echo ""
echo "Cluster: $CLUSTER_NAME"
echo "Project: $PROJECT"
echo "REGION: $REGION"
echo "ZONE: $ZONE"
echo ""


# Hyperdisk balanced is set as default disk type for c4a machine type
#  --disk-type=pd-ssd \

# Using node auto-provisioning
# https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning

# List of all available and valid image  types
# gcloud container get-server-config

# List all available machine types for zone Singapore
# gcloud compute machine-types list | grep 'us-east4-*' | grep 'c4a-standard-*'

# Choosing a minimum CPU platform
# https://cloud.google.com/kubernetes-engine/docs/how-to/min-cpu-platform

# Setting up NodeLocal DNSCache
#https://cloud.google.com/kubernetes-engine/docs/how-to/nodelocal-dns-cache

# Configuring vertical Pod autoscaling
# https://cloud.google.com/kubernetes-engine/docs/how-to/vertical-pod-autoscaling
