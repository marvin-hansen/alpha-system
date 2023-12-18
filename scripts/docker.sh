# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cp specs/src/services/cmdb/Dockerfile Dockerfile_cmdb
command docker build -t cmdb:latest -f Dockerfile_cmdb .
command rm Dockerfile_cmdb


command cp specs/src/services/dbgw/Dockerfile Dockerfile_dbgw
command docker build -t dbgw:latest -f Dockerfile_dbgw .
command rm Dockerfile_dbgw


command cp specs/src/services/qdgw/Dockerfile Dockerfile_qdgw
command docker build -t qdgw:latest -f Dockerfile_qdgw .
command rm Dockerfile_qdgw


command cp specs/src/services/smdb/Dockerfile Dockerfile_smdb
command docker build -t smdb:latest -f Dockerfile_smdb .
command rm Dockerfile_smdb
