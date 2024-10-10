# Sealed Secrets

In order to store secrets safely in a public or private Git repository, you can use Bitnami’s 
sealed-secrets controller and encrypt your Kubernetes Secrets into SealedSecrets. 
The sealed secrets can be decrypted only by the controller running in your cluster 
and nobody else can obtain the original secret, even if they have access to the Git repository.


At startup, the sealed-secrets controller generates a 4096-bit RSA key pair 
and persists the private and public keys as Kubernetes secrets in the flux-system namespace.

## Install the kubeseal CLI:

```shell
brew install kubeseal
```

kubectl create secret docker-registry artifactory-auth --docker-server=asia-northeast1-docker.pkg.dev/future-309012/image-repo --docker-username=_json_key --docker-password="$(cat future.json)" --dry-run=client -o yaml > artifactory-auth.yaml

kubeseal --format=yaml --cert=pub-sealed-secrets.pem < artifactory-auth.yaml > artifactory-auth-sealed.yaml

## Obtain public key to encrypt secrets

```shell
kubeseal --fetch-cert \
--controller-name=sealed-secrets-controller \
--controller-namespace=flux-system \
> pub-sealed-secrets.pem
```

## Encrypt secrets

1) Generate a Kubernetes secret manifest with kubectl:

```bash
kubectl -n default create secret generic basic-auth \
--from-literal=user=admin \
--from-literal=password=change-me \
--dry-run=client \
-o yaml > basic-auth.yaml
```

2) Encrypt the secret with kubeseal:

```shell
kubeseal --format=yaml --cert=pub-sealed-secrets.pem < basic-auth.yaml > basic-auth-sealed.yaml
```

3) Delete the plain secret and apply the sealed one:

```shell
kubectl delete secret basic-auth
kubectl apply -f basic-auth-sealed.yaml
```

Verify that the sealed-secrets controller has created the basic-auth Kubernetes Secret:

```shell
kubectl -n default get secrets basic-auth
```

## GitOps

Add later


## Resources:
* [Sealed secrets operator ](https://github.com/bitnami-labs/sealed-secrets)
* [Sealed secrets in FluxCD](https://fluxcd.io/flux/guides/sealed-secrets/)