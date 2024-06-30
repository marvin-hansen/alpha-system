# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Bazel file formatting (Installed via homebrew)
# https://github.com/bazelbuild/buildtools
buildifier -r *.bazel

# Rust code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all

# Load current Bazel target
current=$(cat current.txt)

#echo "$current"
command bazel build "$current"