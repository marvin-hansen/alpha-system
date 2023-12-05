# SPDX-License-Identifier: MIT
# Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.

# bin/bash
set -o errexit
set -o nounset
set -o pipefail

command cp services/dbgw/Dockerfile Dockerfile_DBGW

command docker build -t dbgw:latest -f Dockerfile_DBGW .

command rm Dockerfile_DBGW