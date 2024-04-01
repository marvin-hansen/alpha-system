# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Pull all images from remote registry

# Pull build & dev containers
command docker pull asia-northeast1-docker.pkg.dev/future-309012/image-repo/rust-build:1.77-alpine
command docker pull asia-northeast1-docker.pkg.dev/future-309012/image-repo/rust-scratch:1.77-alpine
# Pull system containers
command docker pull asia-northeast1-docker.pkg.dev/future-309012/image-repo/cmdb_arm64:latest
command docker pull asia-northeast1-docker.pkg.dev/future-309012/image-repo/dbgw_arm64:latest
command docker pull asia-northeast1-docker.pkg.dev/future-309012/image-repo/smdb_arm64:latest
command docker pull asia-northeast1-docker.pkg.dev/future-309012/image-repo/symdb_arm64:latest