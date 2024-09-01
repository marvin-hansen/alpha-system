# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Check for outdated dependencies
# https://github.com/kbknapp/cargo-outdated
MIGRATION_DATA="migrations" cargo outdated --workspace


# Scan for unused dependencies
# https://crates.io/crates/cargo-udeps
MIGRATION_DATA="migrations" cargo +nightly udeps --all-targets


# Check a package and all of its dependencies for errors.
# https://doc.rust-lang.org/cargo/commands/cargo-check.html
MIGRATION_DATA="migrations" cargo check --all-targets


# Check for linter errors
# https://github.com/rust-lang/rust-clippy
MIGRATION_DATA="migrations" cargo clippy --all-targets


# Check code formatting
# https://github.com/rust-lang/rustfmt
MIGRATION_DATA="migrations" cargo fmt --all --check


# The problem of sub-sub dependencies simply not patching known vulnerabilities
# remains unsolved hence sec scanning has been disabled for the time being.
# Scan for security vulnerabilities
# https://crates.io/crates/cargo-audit
# MIGRATION_DATA="migrations" cargo audit
