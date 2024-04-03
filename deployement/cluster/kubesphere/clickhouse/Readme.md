# ClickHouse Operator Installation

## Install k8s CH operator

> kubectl apply -f https://raw.githubusercontent.com/Altinity/clickhouse-operator/master/deploy/operator/clickhouse-operator-install-bundle.yaml


## Verify installation

> kubectl get pods -n kube-system | grep clickhouse

## Apply CH manifest 


