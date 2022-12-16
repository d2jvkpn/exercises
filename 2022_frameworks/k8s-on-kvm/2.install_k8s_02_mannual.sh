#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


#### 1. add paths to $PATH
mkdir -p /opt/bin

cat >> ~/.bash_aliases << 'EOF'
export PATH=/opt/bin:$PATH

for d in $(ls -d /opt/*/ 2> /dev/null | sed 's#/$##'); do
  [[ -d $d ]] || continue
  [[ -d $d/bin ]] && d=$d/bin
  export PATH=$d:$PATH
done

for d in $(ls -d ~/apps/*/ 2>/dev/null | sed 's#/$##'); do
    test -d $d/bin && d=$d/bin
    export PATH=$d:$PATH
done

alias nerdctl='nerdctl -n k8s.io'
EOF


#### 2. install nerdctl
version=1.1.0
# wget https://github.com/containerd/nerdctl/releases/download/v${version}/nerdctl-full-${version}-linux-amd64.tar.gz

mkdir -p /opt/nerdctl-full-$version-linux-amd64
tar -xf nerdctl-full-$version-linux-amd64.tar.gz -C /opt/nerdctl-full-$version-linux-amd64

mkdir -p /opt/cni/bin
cp -r /opt/nerdctl-full-$version-linux-amd64/libexec/cni/* /opt/cni/bin

nerdctl -n k8s.io images

nerdctl ps -a
nerdctl -n k8s.io ps -a


#### 3. install yq
version=4.30.5
https://github.com/mikefarah/yq/releases/download/v$version/yq_linux_amd64.tar.gz
tar -xf yq_linux_amd64.tar.gz
mv yq_linux_amd64 /opt/bin/yq
./install-man-page.sh
rm yq.1 install-man-page.sh


#### 4. load images
ls kubernetes_v1.26 calico ingress-nginx

for f in $(ls kubernetes_v1.26/*.tar.gz); do
    echo ">>> load image from $f"
    pigz -dc $f | ctr -n=k8s.io image import -
done

for f in $(ls calico/*.tar.gz); do
    echo ">>> load image from $f"
    pigz -dc $f | ctr -n=k8s.io image import -
done

for f in $(ls ingress-nginx/*.tar.gz); do
    echo ">>> load image from $f"
    pigz -dc $f | ctr -n=k8s.io image import -
done
# pigz -dc $f | docker load
nerdctl -n k8s.io images

rm -r calico ingress-nginx kubernetes_v1.26

# calico/cni:v3.23.3
# calico/kube-controllers:v3.23.3
# calico/node:v3.23.3
# k8s.gcr.io/coredns/coredns:v1.8.6
# k8s.gcr.io/etcd:3.5.3-0
# k8s.gcr.io/kube-apiserver:v1.25.1
# k8s.gcr.io/kube-controller-manager:v1.25.1
# k8s.gcr.io/kube-proxy:v1.25.1
# k8s.gcr.io/kube-scheduler:v1.25.1
# k8s.gcr.io/pause:3.7
# registry.k8s.io/ingress-nginx/controller:v1.3.0
# registry.k8s.io/ingress-nginx/kube-webhook-certgen:v1.1.1
# registry.k8s.io/ingress-nginx/kube-webhook-certgen:v1.3.0
