# Copyright (c) 2021. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com

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
echo "* Configuring cluster DNS Service for scratch containers"
DNS=$(kubectl get svc kube-dns -n kube-system -o jsonpath={.spec.clusterIP})

# Stores DNS SERVER IP address in cluster wide ENV variable
# See manifests/deployment.yml for ENV variables that are accessible from within the container.
echo "*  Store DNS configuration"
command kubectl create secret generic dns-access --from-literal=DNS_SERVER="$DNS"

echo "* Configuring cluster DNS completed"
