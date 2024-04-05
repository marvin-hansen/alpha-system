# ClickHouse Installation

## Install Helm Chart

> helm repo add bitnami https://charts.bitnami.com/bitnami
> helm repo update

## Install ClickHouse 

Default config: 

> helm install clickhouse bitnami/clickhouse --namespace quant-engine --create-namespace

Custom config: 

> helm install clickhouse bitnami/clickhouse --namespace quant-engine -f values.yaml
helm install clickhouse bitnami/clickhouse --namespace default -f values.yaml
NAME: clickhouse
LAST DEPLOYED: Fri Apr  5 15:45:58 2024
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
NOTES:
CHART NAME: clickhouse
CHART VERSION: 6.0.0
APP VERSION: 24.3.2

** Please be patient while the chart is being deployed **

ClickHouse is available in the following address:

    kubectl port-forward --namespace default svc/clickhouse 9000:9000 &

Credentials:

    echo "Username      : default"
    echo "Password      : $(kubectl get secret --namespace default clickhouse -o jsonpath="{.data.admin-password}" | base64 -d)"

User: default
PW: OPnWCVBQR7

## Verify deployment

> kubectl get pods -n quant-engine

> kubectl get pods -n quant-engine -l app.kubernetes.io/name=clickhouse

## Access the basic UI

> kubectl port-forward  svc/clickhouse 8123:8123

Open in a browser: 

http://localhost:8123/play 


Sources:

https://medium.com/@larry311012/building-and-testing-a-modern-data-analytics-suite-locally-with-kubernetes-before-launching-to-793ba4d46917