# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command docker build --build-arg="SERVICE_NAME=cmdb" -t cmdb_arm64:latest -f Dockerfile .
command docker build --build-arg="SERVICE_NAME=dbgw" -t dbgw_arm64:latest -f Dockerfile .
command docker build --build-arg="SERVICE_NAME=smdb" -t smdb_arm64:latest -f Dockerfile .
command docker build --build-arg="SERVICE_NAME=symdb" -t symdb_arm64:latest -f Dockerfile .
