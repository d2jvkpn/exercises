#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


#### deploy
kubectl apply -f hello02_deployment.yaml
kubectl apply -f hello02-clusterip.yaml
kubectl apply -f hello02-ingress.yaml

exit
#### rollout
image=registry.cn-shanghai.aliyuncs.com/d2jvkpn/hello

docker tag $image:main $image:2021-11-19
kubectl set image deployment/hello02 hello02=$image:2021-11-19

kubectl get pods

exit
#### scale
kubectl scale deployment/hello02 --replicas=2
kubectl autoscale deployment/hello02 --cpu-percent=50 --min=2 --max=10
