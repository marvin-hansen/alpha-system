# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cp queng_specs/service_specs/src/services/cmdb/Dockerfile Dockerfile_cmdb
command docker build -t cmdb:latest -f Dockerfile_cmdb .
command rm Dockerfile_cmdb


command cp queng_specs/service_specs/src/services/dbgw/Dockerfile Dockerfile_dbgw
command docker build -t dbgw:latest -f Dockerfile_dbgw .
command rm Dockerfile_dbgw


command cp queng_specs/service_specs/src/services/ims_data/binance/Dockerfile Dockerfile_bianance_data
command docker build -t bianance_data:latest -f Dockerfile_bianance_data .
command rm Dockerfile_bianance_data


# Not implemented
#command cp queng_specs/service_specs/src/services/qdgw/Dockerfile Dockerfile_qdgw
#command docker build -t qdgw:latest -f Dockerfile_qdgw .
#command rm Dockerfile_qdgw


command cp queng_specs/service_specs/src/services/smdb/Dockerfile Dockerfile_smdb
command docker build -t smdb:latest -f Dockerfile_smdb .
command rm Dockerfile_smdb

command cp queng_specs/service_specs/src/services/symdb/Dockerfile Dockerfile_symdb
command docker build -t symdb:latest -f Dockerfile_symdb .
command rm Dockerfile_symdb