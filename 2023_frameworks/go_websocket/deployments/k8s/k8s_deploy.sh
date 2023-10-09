#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


#### 1. execute on every k8s node
apt update && apt -y upgrade && apt install -y nfs-kernel-server nfs-common

#### 2. execute on k8s control-plane node (k8scp)
p=/data/nfs
mkdir -p $p && chmod 1777 $p

[[ -f /etc/exports ]] && cp /etc/exports /etc/exports.bk
echo "$p *(rw,sync,no_root_squash,subtree_check)" >> /etc/exports

exportfs -ra
showmount -e k8scp
echo software > $p/hello.txt

#### 3. create a nfs persistentVolume on k8scp node
cat > pv-nfs.yaml << EOF
apiVersion: v1
kind: PersistentVolume
metadata:
  name: pv-nfs

spec:
  capacity: {storage: 2Gi}
  accessModes: [ReadWriteMany]
  persistentVolumeReclaimPolicy: Retain

  nfs:
    path: /data/nfs
    server: k8scp
    readOnly: false
EOF

kubectl create -f pv-nfs.yaml
kubectl get pv

#### 4. create a persistentVolumeClaim
cat > pvc-nfs.yaml << EOF
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: pvc-nfs
spec:
  accessModes: [ReadWriteMany]
  resources:
    requests:
      storage: 1Gi
EOF

kubectl create -f pvc-nfs.yaml
kubectl get pvc

#### 5. create a deployment (hello02)
kubectl apply -f ${_path}/k8s_deploy.yaml
kubectl get deploy

#### 6. create a ClusterIP service (hello02-clusterip) for the deployment (hello02)
kubectl apply -f ${_path}/k8s_service.yaml
kubectl get services

#### 7. create a ingress (hello02-ingress) for hello02-clusterip
kubectl apply -f ${_path}/k8s_ingress.yaml
kubectl get ingress

curl localhost/api/open/ping # curl k8scp/api/open/ping

#### 8. create a nodeport service (hello02-nodeport) for the deploymet
kubectl apply -f ${_path}/k8s_nodeport.yaml
kubectl get services
