# Copyright (c) 2024. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com

# bin/bash
set -o errexit
set -o nounset
set -o pipefail

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
# command rm secret.yaml

# Null variable
USER=""
PASSWORD=""

# Create temporary file
command cp secret-template.yaml root-secret.yaml

# printf postgres | base64
USER="cG9zdGdyZXM="
# printf postgres | base64
PASSWORD="cG9zdGdyZXM="
SECRETNAME="postgres-admin"
YAML_FILE="root-secret.yaml"

# Replace TOPSECRET with the value from PASSWORD
sed -i '' "s/TOPSECRET/$PASSWORD/g" "$YAML_FILE"

# Replace USERNAME with the value from USER
command sed -i '' "s/USERNAME/$USER/g" "$YAML_FILE"

# Replace SECRETID with the value from SECRETNAME
command sed -i '' "s/SECRETID/$SECRETNAME/g" "$YAML_FILE"

# Install secret in cluster
command kubectl apply -f root-secret.yaml

# Remove temporary file
# command rm root-secret.yaml
# Null variable
USER=""
PASSWORD=""

echo "► Configuring cluster secrets completed"
