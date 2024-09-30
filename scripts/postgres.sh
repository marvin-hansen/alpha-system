# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command docker run --name postgres-5432 -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres:17-alpine3.20
