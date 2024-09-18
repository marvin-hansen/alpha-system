#
# Copyright (c) 2021. Marvin Friedrich Lars Hansen. All Rights Reserved. Contact: marvin.hansen@gmail.com
#

# bin/bash
set -o errexit
set -o nounset
set -o pipefail

 echo "* Delete previously generated credentials"
 command kubectl delete secret cmdb-creds
 command kubectl delete secret imdb-creds
 command kubectl delete secret mddb-creds
 command kubectl delete secret smdb-creds

echo "* Generating user name and password"
CMDB_USER='cmdbuser'
CMDB_PASSWORD=$(openssl rand -base64 32)

IMDB_USER='imdbuser'
IMDB_PASSWORD=$(openssl rand -base64 32)

MDDB_USER='mddbuser'
MDDB_PASSWORD=$(openssl rand -base64 32)

SMDB_USER='smdbuser'
SMDB_PASSWORD=$(openssl rand -base64 32)

echo "* Create secret to store user and password"
command kubectl create secret generic cmdb-creds --from-literal=CMDB_USER="$CMDB_USER" --from-literal=CMDB_PASSWORD="$CMDB_PASSWORD"
command kubectl create secret generic imdb-creds --from-literal=IMDB_USER="$IMDB_USER" --from-literal=IMDB_PASSWORD="$IMDB_PASSWORD"
command kubectl create secret generic mddb-creds --from-literal=MDDDB_USER="$MDDB_USER" --from-literal=MDDDB_PASSWORD="$MDDB_PASSWORD"
command kubectl create secret generic smdb-creds --from-literal=SMDB_USER="$SMDB_USER" --from-literal=SMDB_PASSWORD="$SMDB_PASSWORD"

DB_HOST="svc/postgres-cluster-rw"

SQL_USERS="
    CREATE USER $CMDB_USER with encrypted password '$CMDB_PASSWORD';
    CREATE USER $IMDB_USER with encrypted password '$IMDB_PASSWORD';
    CREATE USER $MDDB_USER with encrypted password '$MDDB_PASSWORD';
    CREATE USER $SMDB_USER with encrypted password '$SMDB_PASSWORD';
"

command kubectl exec -it "$DB_HOST" -- psql -c "$SQL_USERS"

SQL_SCHEMAS="
    CREATE SCHEMA IF NOT EXISTS cmdb AUTHORIZATION $CMDB_USER;
    CREATE SCHEMA IF NOT EXISTS imdb AUTHORIZATION $IMDB_USER;
    CREATE SCHEMA IF NOT EXISTS mddb AUTHORIZATION $MDDB_USER;
    CREATE SCHEMA IF NOT EXISTS smdb AUTHORIZATION $SMDB_USER;
"

command kubectl exec -it "$DB_HOST" -- psql -c "$SQL_SCHEMAS"

# For access to the secrets from a client authenticated to the cluster:
# command kubectl get secret --namespace default cmdb-creds -o jsonpath='{.data.CMDB_USER}' | base64 --decode -
# command kubectl get secret --namespace default cmdb-creds -o jsonpath='{.data.CMDB_PASSWORD}' | base64 --decode -

# Clear all generated credentials
SQL_USERS=""
SQL_SCHEMAS=""
CMDB_USER=''
CMDB_PASSWORD=''
IMDB_USER=''
IMDB_PASSWORD=''
MDDB_USER=''
MDDB_PASSWORD=''
SMDB_USER=''
SMDB_PASSWORD=''

echo "Credentials generated and stored in kubernetes secrets"

# An Introduction to Kubernetes Secrets and ConfigMaps
# https://opensource.com/article/19/6/introduction-kubernetes-secrets-and-configmaps

# Kubernetes Fundamentals, Part 3: How to Use Kubernetes Secrets
# https://blog.newrelic.com/engineering/how-to-use-kubernetes-secrets/

# Define an environment variable for a container
# https://kubernetes.io/docs/tasks/inject-data-application/define-environment-variable-container/

# Using Kubernetes Secrets as Environment Variables
# https://medium.com/faun/using-kubernetes-secrets-as-environment-variables-5ea3ef7581ef