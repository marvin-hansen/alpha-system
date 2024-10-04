# Copyright (c) 2024. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com

# bin/bash
set -o errexit
set -o nounset
set -o pipefail

echo ""
echo "==============================="
echo " Configure cluster DNS:  "
echo "==============================="
echo ""

# User name only consists of letters, no numbers and special characters to avoid any problems.
echo "► Configuring cluster DNS Service"
DNS=$(kubectl get svc kube-dns -n kube-system -o jsonpath={.spec.clusterIP})

# Stores DNS SERVER IP address in cluster wide ENV variable
# See manifests/deployment.yml for ENV variables that are accessible from within the container.
echo "►  Store DNS configuration"
command kubectl create secret generic dns-access --from-literal=DNS_SERVER="$DNS"

echo "► Configuring cluster DNS completed"

echo ""
echo "==============================="
echo " Configure cluster secrets:  "
echo "==============================="
echo ""

echo "► Generating DB user name and password"
USER='dbgwuser'
PASSWORD=$(openssl rand -base64 32)

# store DB user & password in kubernetes secret accessed only by the SMX container.
# See manifests/deployment.yml for ENV variables that are accessible from within the container.
echo "► Create secret to store PG user and password"
command kubectl create secret generic postgres-user \
  --from-literal=username="$USER" \
  --from-literal=password="$PASSWORD"

echo "► Generating DB admin and password"
USER='postgres'
PASSWORD="cG9zdGdyZXM="

command kubectl create secret generic postgres-admin \
  --from-literal=username="$USER" \
  --from-literal=password="$PASSWORD"

echo "► Secrets configuration completed"

echo ""
echo "==============================="
echo " Configure Flux cont. Delivery:"
echo "==============================="
echo ""

command flux bootstrap github \
          --components-extra=image-reflector-controller,image-automation-controller \
          --owner=marvin-hansen \
          --repository=quant-engine \
          --branch=main \
          --path=delivery/clusters/staging \
          --personal \
          --token-auth # https://github.com/fluxcd/flux2/issues/2509

echo "► Flux configuration completed"

command flux check

echo "► Flux check completed"

command git pull

echo "► Git pull completed"

echo "!! Cluster setup completed !! "