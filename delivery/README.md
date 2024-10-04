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

Install Flux with the image automation components:

```shell
flux bootstrap github \
  --components-extra=image-reflector-controller,image-automation-controller \
  --owner=marvin-hansen \
  --repository=quant-engine \
  --branch=main \
  --path=delivery/clusters \
  --personal
```
The bootstrap command creates a repository if one doesn’t exist, and commits the manifests for the Flux components 
to the default branch at the specified path. It then configures the target cluster to synchronize 
with the specified path inside the repository.

## Update K8s Operators

A) Add customization to git
B) Commit and push to origin
C) Reconcile k8s cluster with git

```shell
flux reconcile source git flux-system
```


Resources:
* https://www.youtube.com/watch?v=vp-oFksFoZs