#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


deploy=$1
# image=registry.cn-shanghai.aliyuncs.com/d2jvkpn/hello:main
image=$(kubectl get deploy/$deploy -o json | jq -r .spec.template.spec.containers[0].image)
# image=$(kubectl get deploy/$deploy -o yaml | yq eval .spec.template.spec.containers[0].image)

echo ">>> Waiting for previous rollout"
kubectl rollout status deployment/$deploy

if [[ "$image" == *-xx ]]; then new=${image%-xx}; else new=${image}-xx; fi

echo -e ">>> $deploy current image: $image\n    new image: $new"
kubectl set image deployment/$deploy $deploy=${new}
kubectl rollout status deployment/$deploy

exit
kubectl rollout undo deployment/$deploy
