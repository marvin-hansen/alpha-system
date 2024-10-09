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

echo "► Configuring cluster secrets"

# Create temporary file
command cp secret-template.yaml secret.yaml

# Generate random password
# printf dbgwuser | base64
USER="ZGJnd3VzZXI="
PASSWORD=$(openssl rand -base64 32)
SECRETNAME="postgres-app"
YAML_FILE="secret.yaml"

# Replace TOPSECRET with the value from PASSWORD
sed -i '' "s/TOPSECRET/$PASSWORD/g" "$YAML_FILE"

# Replace USERNAME with the value from USER
command sed -i '' "s/USERNAME/$USER/g" "$YAML_FILE"

# Replace SECRETID with the value from SECRETNAME
command sed -i '' "s/SECRETID/$SECRETNAME/g" "$YAML_FILE"

# Install secret in cluster
command kubectl apply -f secret.yaml

# Remove temporary file
command rm secret.yaml

# Null variable
USER=""
PASSWORD=""

echo "► Configuring cluster secrets completed"


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

command git pull

echo "► Git pull completed"

echo "!! Cluster setup completed !! "