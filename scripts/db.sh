# bin/bash
set -o errexit
set -o nounset
set -o pipefail

# Install surreal DB:
# https://docs.surrealdb.com/docs/installation/macos

# brew install surrealdb/tap/surreal

# For the first time run.  This will create a root user and create a directory to store the db files.
#surreal start --user root --pass root --bind 0.0.0.0:8000 file:db/mydatabase.db

# The root user is stored in the DB, therefore we can start the server without parameters.
surreal start file:db/mydatabase.db
