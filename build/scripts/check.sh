#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Check for outdated dependencies
# https://github.com/kbknapp/cargo-outdated
command cargo outdated --workspace

# Scan for unused dependencies
# https://crates.io/crates/cargo-udeps
command cargo +nightly udeps --all-targets


# Check a package and all of its dependencies for errors.
# https://doc.rust-lang.org/cargo/commands/cargo-check.html
command cargo check --all-targets


# Check for linter errors
# https://github.com/rust-lang/rust-clippy
command cargo clippy --all-targets


# Check code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all --check


# The problem of sub-sub dependencies simply not patching known vulnerabilities
# remains unsolved hence sec scanning has been disabled for the time being.
# Scan for security vulnerabilities
# https://crates.io/crates/cargo-audit
# command cargo audit
