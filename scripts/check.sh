# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Check for outdated dependencies
# https://github.com/kbknapp/cargo-outdated
command cargo outdated --workspace

# Check for linter errors
# https://github.com/rust-lang/rust-clippy
command cargo clippy --all-targets

# Check code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all --check

