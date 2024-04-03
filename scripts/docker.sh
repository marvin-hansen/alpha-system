# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=cmdb"  --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/cmdb_arm64:latest -f Dockerfile_Local .
command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=dbgw"  --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/dbgw_arm64:latest -f Dockerfile_Local .
command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=smdb"  --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/smdb_arm64:latest -f Dockerfile_Local .
command docker build --platform=linux/arm64 --build-arg="SERVICE_NAME=symdb" --build-arg="BUILD_TARGET=aarch64-unknown-linux-musl" -t asia-northeast1-docker.pkg.dev/future-309012/image-repo/symdb_arm64:latest -f Dockerfile_Local .

command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/cmdb_arm64:latest
command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/dbgw_arm64:latest
command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/smdb_arm64:latest
command docker push asia-northeast1-docker.pkg.dev/future-309012/image-repo/symdb_arm64:latest
