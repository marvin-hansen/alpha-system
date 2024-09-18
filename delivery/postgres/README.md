# Install

1) Install operator

```shell
kubectl apply --server-side -f cnpg-1.24.0.yaml
```

2) Verify oprerator

```shell
kubectl get deployment -n cnpg-system cnpg-controller-manager
```

!!! Warning If you are deploying CloudNativePG on GKE and get an error (... failed to call webhook...), 
be aware that by default traffic between worker nodes and control plane is blocked by the firewall except for a few specific ports, 
as explained in the official docs and by this issue. You'll need to either change the targetPort in the webhook service, 
to be one of the allowed ones, or open the webhooks' port (9443) on the firewall.

https://github.com/cloudnative-pg/cloudnative-pg/blob/main/docs/src/installation_upgrade.md


3) Install postgres cluster

```shell
kubectl apply -f cluster.yaml
```

4) Verify postgres cluster

```shell
kubectl get pods 
```

or
```shell
kubectl get pods -l cnpg.io/cluster=postgres-cluster
```

5) Configure users and schemas

```shell
./create_user_secrets.sh
```

SRC:
https://github.com/cloudnative-pg/cloudnative-pg/blob/main/docs/src/quickstart.md

https://github.com/cloudnative-pg/cloudnative-pg/blob/main/docs/src/samples/cluster-example-secret.yaml
