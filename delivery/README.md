# Continuous Delivery with Flux

## Prerequisites

You will need a Kubernetes cluster version 1.16 or newer and kubectl version 1.18. 
For a quick local test, you can use Kubernetes kind. Any other Kubernetes setup will work as well.

## GitHub PAT


You’ll need a GitHub account and a personal access token that can create repositories (check all permissions under repo).

Scope:
* Only affected repository

Permissions:
* Metadata: read (Default)
* Administration: read, write (Required)
* Content: read, write (Required)

Export your GitHub personal access token:

```shell
export GITHUB_TOKEN=<your-token>
```

## Install Flux

Install Flux with the image automation components via the setup script:

```shell
chmod +x ./setup_staging_cluster.sh && ./setup_staging_cluster.sh
```

Check installation:

```shell
flux check 

kubectl get pods --all-namespaces
```

## Update K8s Operators

A) Add customization to git
B) Commit and push to origin
C) Reconcile k8s cluster with git

```shell
flux reconcile source git flux-system
```
Double check pods were created in k8s:

```shell
kubectl get pods --all-namespaces
```

# Pre-installed operators:

* [CNPG Postgres cluster operator](https://cloudnative-pg.io/)
* [Sealed secrets operator ](https://github.com/bitnami-labs/sealed-secrets)


## Resources:
* https://www.youtube.com/watch?v=vp-oFksFoZs