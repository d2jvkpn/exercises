#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


####
cat /etc/kubernetes/manifests/kube-controller-manager.yaml | grep -i cluster-cidr
# 10.96.2.10/16

kubectl get nodes
kubectl patch node NODE01 -p '{"spec":{"podCIDR":"10.96.2.10/16"}}'

# https://github.com/flannel-io/flannel
wget https://raw.githubusercontent.com/flannel-io/flannel/master/Documentation/kube-flannel.yml

sed -i 's#10.244.0.0/16#10.96.2.10/16#' kube-flannel.yml

kubectl apply -f kube-flannel.yml

kubectl get pods --all-namespaces


####
# kubect apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml

node=$(hostname | tr '[:upper:]' '[:lower:]')
kubectl label nodes/$node ingress-ready=true
kubectl taint nodes --all node-role.kubernetes.io/control-plane-
# kubectl label nodes/$node nodePool=cluster


####
kubectl -n prod create secret tls localdev.me  \
  --key ~/.acme.sh/localdev.me/localdev.me.key \
  --cert ~/.acme.sh/localdev.me/localdev.me.cer

kubectl -n prod get secret/localdev.me

kubectl -n prod create secret tls localdev.me --dry-run=client   \
   --key ~/.acme.sh/localdev.me/localdev.me.key \
   --cert ~/.acme.sh/localdev.me/localdev.me.cer -o yaml |
   kubectl apply -f -

####
kubectl create secret docker-registry my-registry \
  --docker-server="REGISTRY.SITE.COM" --docker-email="EMAIL" \
  --docker-username="USERNAME" --docker-password="PASSWORD"
