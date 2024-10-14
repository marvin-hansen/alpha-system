# Image automation

## ImageRepository

Before Flux can monitor a registry for new images, it must know what repository to monitor and that's the purpose of the Image Repository resource.

## Create an Image Repository

To create an Image Repository, run the following command:
Ensure the  secret artifact-registry exists and contains
the authentication to the image registry.

```bash
apiVersion: image.toolkit.fluxcd.io/v1beta2
kind: ImageRepository
metadata:
  name: dbgw-image
  namespace: flux-system
spec:
  secretRef:
    name: artifact-registry
  image: asia-northeast1-docker.pkg.dev/future-309012/image-repo/dbgw
  interval: 1m0s
```

## Create an Image Policy

Next, you'll create an image policy. An image policy is the rules Flux follows to determine what images are newer. 

```shell
apiVersion: image.toolkit.fluxcd.io/v1beta2
kind: ImagePolicy
metadata:
  name: dbgw-image-policy
  namespace: flux-system
spec:
  imageRepositoryRef:
    name: dbgw-image
  filterTags:
    # ${SHA:0:7}-$(date +%s) (numerical):
    pattern: '[a-fA-F0-9]+-(?P<timestamp>.*)'
    extract: '$timestamp'
  policy:
    alphabetical:
      order: asc
```

## Git commit

Commit and push to origin.

## Verify 

```shell
flux reconcile source git flux-system
```
Double check pods were created in k8s:

```shell
flux get image repository -n default
```

```shell
kubectl describe imagerepositories dbgw-image -n default
```

```shell
flux get image policy -n default
```

## image updates

Edit the manifest.yaml and add a marker to tell Flux which policy to use when updating the container image:

```yaml
containers:
- name: dbgw
  image: asia-northeast1-docker.pkg.dev/future-309012/image-repo/dbgw:9e9da5d1-20241013095625 # {"$imagepolicy": "default:dbgw-image-policy"}
  imagePullPolicy: IfNotPresent
```

Create an ImageUpdateAutomation to tell Flux which Git repository to write image updates to:


```shell
apiVersion: image.toolkit.fluxcd.io/v1beta2
kind: ImageUpdateAutomation
metadata:
  name: flux-system
  namespace: flux-system
spec:
  interval: 10m
  sourceRef:
    kind: GitRepository
    name: flux-system
  git:
    checkout:
      ref:
        branch: main
    commit:
      author:
        email: fluxcdbot@users.noreply.github.com
        name: fluxcdbot
      messageTemplate: '{{range .Changed.Changes}}{{print .OldValue}} -> {{println .NewValue}}{{end}}'
    push:
      branch: main
  update:
    path: ./delivery/clusters/staging
    strategy: Setters
```

## Resources

* https://fluxcd.io/flux/components/image/imageupdateautomations/
* https://fluxcd.io/flux/guides/image-update/
* https://fluxcd.io/flux/guides/image-update/#imagepolicy-examples
* https://dev.to/azure/configure-image-automation-with-fluxcd-1ecc