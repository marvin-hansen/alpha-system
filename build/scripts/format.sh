#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Bazel file formatting (Installed via homebrew)
# https://github.com/bazelbuild/buildtools
buildifier -r MODULE.bazel BUILD.bazel thirdparty/BUILD.bazel
buildifier -r build build/images tools
buildifier -r queng_*

# Rust code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all