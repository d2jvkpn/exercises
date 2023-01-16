#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


version=$1

#### 1. apt install pacakges
apt-get update && apt -y upgrade

apt-get install -y socat conntrack nfs-kernel-server nfs-common nftables jq apt-transport-https \
  ca-certificates curl gnupg lsb-release


#### 2. install kubelet kubeadm kubectl and config k8s
# version=1.26.0
curl https://mirrors.aliyun.com/kubernetes/apt/doc/apt-key.gpg | apt-key add -

cat > /etc/apt/sources.list.d/kubernetes.list <<EOF
deb https://mirrors.aliyun.com/kubernetes/apt/ kubernetes-xenial main
EOF

apt update && apt install -y kubelet=${version}-00 kubeadm=${version}-00 kubectl=${version}-00

# google source mirror
# curl -fsSLo /usr/share/keyrings/kubernetes-archive-keyring.gpg https://packages.cloud.google.com/apt/doc/apt-key.gpg
# echo "deb [signed-by=/usr/share/keyrings/kubernetes-archive-keyring.gpg] https://apt.kubernetes.io/ kubernetes-xenial main" |
#  sudo tee /etc/apt/sources.list.d/kubernetes.list

apt-mark hold kubelet kubeadm kubectl

kubectl version --output=yaml || true

cp /etc/systemd/system/kubelet.service.d/10-kubeadm.conf \
  /etc/systemd/system/kubelet.service.d/10-kubeadm.conf.bk

sed -i '/$KUBELET_EXTRA_ARGS/s/$/ --fail-swap-on=false/' \
  /etc/systemd/system/kubelet.service.d/10-kubeadm.conf

systemctl daemon-reload
systemctl enable kubelet.service
# systemctl status kubelet.service

kubectl completion bash > /etc/bash_completion.d/kubectl
kubeadm config images list

cat <<EOF | tee /etc/modules-load.d/k8s.conf
overlay
br_netfilter
EOF

modprobe overlay
modprobe br_netfilter
lsmod | grep overlay
lsmod | grep br_netfilter

cat <<EOF | tee /etc/sysctl.d/99-kubernetes-cri.conf
net.bridge.bridge-nf-call-iptables  = 1
net.ipv4.ip_forward                 = 1
net.bridge.bridge-nf-call-ip6tables = 1
EOF

sysctl --system


#### 3. install and config containerd
# https://docs.docker.com/engine/install/ubuntu/
mkdir -p /etc/apt/keyrings

curl -fsSL https://download.docker.com/linux/ubuntu/gpg |
   gpg --dearmor -o /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# chmod a+r /etc/apt/keyrings/docker.gpg
apt-get update

# apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
apt-get -y install containerd.io

containerd config default | grep SystemdCgroup
containerd config default | grep sandbox_image


mkdir -p /etc/containerd

pause=$(kubeadm config images list | grep pause)

containerd config default |
  sed '/SystemdCgroup/{s/false/true/}' |
  awk -v pause=$pause '/sandbox_image/{sub("registry.k8s.io/pause.*", pause"\"")} {print}' \
  > /etc/containerd/config.toml

grep pause: /etc/containerd/config.toml

systemctl restart containerd
# systemctl status containerd

cat <<EOF | tee /etc/crictl.yaml
runtime-endpoint: unix:///run/containerd/containerd.sock
image-endpoint: unix:///run/containerd/containerd.sock
timeout: 5
debug: false
pull-image-on-create: false
EOF
