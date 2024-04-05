# Install Zookepeer 

## Install yq

> brew install yq

## Install Zookeper operator 

> chmod +x ./install-zookeeper-operator.sh && ./install-zookeeper-operator.sh


## Verify operator installation:

> kubectl get pods --namespace zookeeper-operator

Expected output:

> NAME                                  READY   STATUS    RESTARTS   AGE
> zookeeper-operator-75f4c6dddc-p2ttg   1/1     Running   0          7m35s

## Install Zookeeper 

> kubectl apply -f zookeeper.yam -n quant-engine

## verify zookeeper installation 

> kubectl get pods --namespace quant-engine