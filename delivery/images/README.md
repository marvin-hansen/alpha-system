# Image automation

## ImageRepository

Before Flux can monitor a registry for new images, it must know what repository to monitor and that's the purpose of the Image Repository resource.

## Create an Image Repository

To create an Image Repository, run the following command:
Ensure the  secret artifact-registry exists and contains
the authentication to the image registry.

```bash
flux create image repository binance-data \
 --image=asia-northeast1-docker.pkg.dev/future-309012/image-repo/binance_data \
 --interval=1m \
 --secret-ref artifact-registry \
 --namespace default \
 --export > ./delivery/images/ims/binance_data.yaml
```

## Create an Image Policy

Next, you'll create an image policy. An image policy is the rules Flux follows to determine what images are newer. 

```shell
apiVersion: image.toolkit.fluxcd.io/v1beta2
kind: ImagePolicy
metadata:
  name: cmdb-image-policy
  namespace: default
spec:
  filterTags:
    # ${SHA:0:7}-$(date +%s) (numerical):
    pattern: '[a-fA-F0-9]+-(?P<timestamp>.*)'
    extract: '$timestamp'
  policy:
    alphabetical:
      order: asc
```

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

## Resources

* https://fluxcd.io/flux/components/image/imageupdateautomations/
* https://fluxcd.io/flux/guides/image-update/
* https://fluxcd.io/flux/guides/image-update/#imagepolicy-examples
* https://dev.to/azure/configure-image-automation-with-fluxcd-1ecc