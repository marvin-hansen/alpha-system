# Configure cluster DNS

The idea is to set the cluster DNS server as a cluster wide secret so
that all services can use that service for cluster DNS resolution.

Get the cluster DNS server:

```shell
kubectl get svc kube-dns -n kube-system -o jsonpath={.spec.clusterIP}
10.96.0.10
```

To make the DNS server available as a secret, just run the configure-cluster-dns.sh script.

```shell
./configure-cluster-dns.sh
```

Test cluster DNS resolution with dns-utils:


```shell
kubectl apply -f dnsutils-yaml

kubectl get pods dnsutils
NAME       READY   STATUS    RESTARTS   AGE
dnsutils   1/1     Running   0          3m5s
```

```shell
kubectl exec -i -t dnsutils -- nslookup kubernetes.default
Server:		10.96.0.10
Address:	10.96.0.10#53

Name:	kubernetes.default.svc.cluster.local
Address: 10.96.0.1
```

Get a list of all services deployed to the cluster:

```shell
kubectl get services
```

Construct FQN DNS:

ServiceName.NameSpace.svc.cluster.local

For example:

* ServiceName:postgres-cluster-rw 
* NameSpace: default

FQN DNS: postgres-cluster-rw.default.svc.cluster.local

Resolve DNS:

```shell
kubectl exec -i -t dnsutils -- nslookup postgres-cluster-rw.default
Server:		10.96.0.10
Address:	10.96.0.10#53

Name:	postgres-cluster-rw.default.svc.cluster.local
Address: 10.107.253.173
```

References:

Debugging DNS Resolution
https://kubernetes.io/docs/tasks/administer-cluster/dns-debugging-resolution/
