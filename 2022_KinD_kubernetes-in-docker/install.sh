#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
# https://kind.sigs.k8s.io/docs/user/quick-start/#installation
# https://github.com/kubernetes-sigs/kind

# curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.15.0/kind-linux-amd64

go install sigs.k8s.io/kind@v0.15.0

kind version

####
kind create cluster --name cluster1
# docker images: kindest/node:v1.25.0

kubectl get
kubectl cluster-info --context kind-cluster1

kind delete cluster --name cluster1

####
mkdir -p  /home/hello/Work/kind/{worker01,worker02,logs}

kind create cluster --config cluster2.yaml # --name cluster2

kubectl cluster-info --context kind-cluster2

kind export logs /home/hello/Work/kind/logs
# # kind delete cluster --name cluster2
