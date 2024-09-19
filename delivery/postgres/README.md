# Install

## Requirements:

* Ensure a cluster is available, kubectl, and Helm are both configured and working.


1) Install operator

```shell
kubectl apply --server-side -f cnpg-1.24.0.yaml

kubectl get deployment -n cnpg-system cnpg-controller-manager
```

!!! Warning If you are deploying CloudNativePG on GKE and get an error (... failed to call webhook...), 
be aware that by default traffic between worker nodes and control plane is blocked by the firewall except for a few specific ports, 
as explained in the official docs and by this issue. You'll need to either change the targetPort in the webhook service, 
to be one of the allowed ones, or open the webhooks' port (9443) on the firewall.

https://github.com/cloudnative-pg/cloudnative-pg/blob/main/docs/src/installation_upgrade.md


2) Install postgres cluster

```shell
kubectl apply -f cluster.yaml

kubectl get pods 
```

3) Test DB connection:

```shell
kubectl port-forward svc/postgres-cluster-rw 5432:5432
```
Connect to postgres cluster:

When enableSuperuserAccess is set to true;
Otherwise, a custom user and password is required.
```shell
psql -h 127.0.0.1 -p 5432 -U postgres -d postgres
```

Optional:

Monitor clusters with Prometheus and Grafana:
https://github.com/cloudnative-pg/cloudnative-pg/blob/main/docs/src/quickstart.md#part-4-monitor-clusters-with-prometheus-and-grafana

SRC:
https://github.com/cloudnative-pg/cloudnative-pg/blob/main/docs/src/quickstart.md

https://github.com/cloudnative-pg/cloudnative-pg/blob/main/docs/src/samples/cluster-example-secret.yaml

https://medium.com/@MetricFire/kubernetes-secrets-management-70e0d269e813
