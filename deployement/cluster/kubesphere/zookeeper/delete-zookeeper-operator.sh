# bin/sh
set -o errexit
set -o nounset
set -o pipefail

ZOOKEEPER_OPERATOR_NAMESPACE=${ZOOKEEPER_OPERATOR_NAMESPACE:-zookeeper-operator}
kubectl delete ns "$ZOOKEEPER_OPERATOR_NAMESPACE"