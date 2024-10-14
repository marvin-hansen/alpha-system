# Copyright (c) 2024. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com

# bin/bash
set -o errexit
set -o nounset
set -o pipefail

echo ""
echo "==============================="
echo " Configure Flux Cont. Delivery:"
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


command flux check

echo "► Flux configuration completed"

command git pull

echo "► Git pull completed"

echo "!! Cluster setup completed !! "