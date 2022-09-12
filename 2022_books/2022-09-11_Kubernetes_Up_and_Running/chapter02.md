### Deploying a Kubernetes Cluster
---

#### 1
- Google Kubernetes Engine, glcoud
- Azure Kubernetes Service, az
- Amazon Web Services, ekstl
- minikube
- Kubernetes in Docker, kind
- 

#### 2
- kubectl version
- kubectl get componentstatuses, This is a good way to verify that your cluster is generally healthy
- kubectl get nodes -o wide --show-labels
- kubectl describe nodes cluster2-control-plane
- kubectl get pods -n kube-system
- kubectl get pods --all-namespaces
- Kubernetes Proxy: The Kubernetes proxy is responsible for routing network traffic to load-
balanced services in the Kubernetes cluster.
  - kubectl --namespace=kube-system get daemonSets/kube-proxy
- Kubernetes DNS: Kubernetes also runs a DNS server, which provides naming and discovery for the
services that are defined in the cluster.
  - kubectl --namespace=kube-system  get deployments/coredns
  - cat /etc/resolv.conf
