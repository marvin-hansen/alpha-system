# Using Google Artifact Registry with k3s

By default, K3S cannot pull from private registries

1) Crete service account with only READ permission from artifict registry
Use the web console b/c CLI somehow doesn't work.

2) Create keys and download as JSON

3) Extract email

4) Format the JSON as one single line.

5) Login to kubesphere as regular user and select a workspace.

6) Create new secret, type registry and specify the following fields.

* Registry Address. The web url ie https://asia-northeast1-docker.pkg.dev

* Username. _json_key
* Password. The one line content of the service account content
* Email of the service account ie container-puller@future-309012.iam.gserviceaccount.com

Sources:
https://cloud.google.com/iam/docs/service-accounts-create#iam-service-accounts-create-console

https://breadnet.co.uk/using-google-artifact-registry-with-k3s/amp/

https://support.count.ly/hc/en-us/articles/4698120212889-Docker-and-Kubernetes-Connecting-to-Private-Artifact-Registry-and-Pulling-Images-with-Authentication-Plugin-Packages

Image Registries in KubeSphere
https://kubesphere.io/docs/v3.4/project-user-guide/configuration/image-registry/