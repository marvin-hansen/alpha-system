# ClickHouse Operator Installation

## Install k8s CH operator

> kubectl apply -f https://raw.githubusercontent.com/Altinity/clickhouse-operator/master/deploy/operator/clickhouse-operator-install-bundle.yaml


## Verify installation

> kubectl get pods -n kube-system | grep clickhouse


## Install zookeeper

Next step, we need to install/configure the Zookeeper. Zookeeper is used to store the replicas meta information.

> kubectl --namespace=kube-system apply -f zookeeper.yaml

## Apply CH manifest 

Sources:
https://chistadata.com/running-clickhouse-cluster-on-minikube-kubernetes/#Minikube_Installation