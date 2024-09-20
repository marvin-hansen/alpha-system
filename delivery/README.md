# Continuous Delivery with Flux

## Prerequisites

You will need a Kubernetes cluster version 1.16 or newer and kubectl version 1.18. 
For a quick local test, you can use Kubernetes kind. Any other Kubernetes setup will work as well.

## GitHub PAT


You’ll need a GitHub account and a personal access token that can create repositories (check all permissions under repo).

Export your GitHub personal access token and username:

```shell
export GITHUB_TOKEN=<your-token>
export GITHUB_USER=<your-username>
```

## Install Flux

Install Flux with the image automation components:

```shell
flux bootstrap github \
  --components-extra=image-reflector-controller,image-automation-controller \
  --owner=$GITHUB_USER \
  --repository=quant-engine \
  --branch=main \
  --path=delivery/clusters \
  --personal
```
The bootstrap command creates a repository if one doesn’t exist, and commits the manifests for the Flux components 
to the default branch at the specified path. It then configures the target cluster to synchronize 
with the specified path inside the repository.

