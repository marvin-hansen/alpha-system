# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command docker build --build-arg="SERVICE_NAME=cmdb" -t cmdb:latest -f Dockerfile .
command docker build --build-arg="SERVICE_NAME=dbgw" -t dbgw:latest -f Dockerfile .
command docker build --build-arg="SERVICE_NAME=binance_data" -t binance_data:latest -f Dockerfile .
command docker build --build-arg="SERVICE_NAME=mddb" -t mddb:latest -f Dockerfile .
command docker build --build-arg="SERVICE_NAME=smdb" -t smdb:latest -f Dockerfile .
