# Deployment

## Kind / Local Testing 

1) Ensure Docker Desktop is installed running

2) Apply installer

kubectl apply -f https://github.com/kubesphere/ks-installer/releases/download/v3.4.1/kubesphere-installer.yaml

3) Install Kubesphere

kubectl apply -f deployement/kubesphere/config.yaml  --dry-run=client

kubectl apply -f deployement/kubesphere/config.yaml


Expected: Output

Console: http://localhost:30880
Account: admin
Password: ...
NOTES：
1. After you log into the console, please check the
   monitoring status of service components in
   "Cluster Management". If any service is not
   ready, please wait patiently until all components
   are up and running.
2. Please change the default password after login.

#####################################################
https://kubesphere.io             2024-04-02 04:16:32
#####################################################