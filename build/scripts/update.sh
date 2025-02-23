#
# Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command echo ""
command echo "Checking for rustup update"
command rustup upgrade


command echo ""
command echo "Checking for rustc stable update"
command rustup update stable


command echo ""
command echo "Running git pull to update local repo"
command git pull
