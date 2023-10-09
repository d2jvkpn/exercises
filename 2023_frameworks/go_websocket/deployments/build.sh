#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


function onExit {
    git checkout dev # --force
    rm -f build.lock
}
mkfifo build.lock
trap onExit EXIT

tag=$1
start_s=$(date +%s)
start_t=$(date +%FT%T%:z)
log_file=logs/devops.log
[[ -f "$log_file" ]] || { mkdir -p logs; echo -e "time\taction\ttag\telapsed" > $log_file; }

BuildBranch=$1
image="registry.cn-shanghai.aliyuncs.com/d2jvkpn/hello"
tag=$BuildBranch

####
b1="$(git rev-parse --abbrev-ref HEAD)" # current branch
uncommitted=$(git status --short)
unpushed=$(git diff origin/$b1..HEAD --name-status)

if [[ $(printenv APP_BuildForce) != "true" ]]; then
    test -z "$uncommitted" || { echo "You have uncommitted changes!"; exit 1; }
    test -z "$unpushed" || { echo "You have unpushed commits!"; exit 1; }
fi

git checkout --force $BuildBranch
BuildTime=$(date +'%FT%T%:z')
CommitHash=$(git rev-parse --verify HEAD) # git log --pretty=format:'%h' -n 1
CommitTime=$(git log -1 --format="%at" | xargs -I{} date -d @{} +%FT%T%:z)
# git tag $git_tag
# git push origin $git_tag

####
echo ">>> pull images..."
#{
#  docker pull --quiet alpine
#  images=$(docker images --filter "dangling=true" --quiet alpine)
#  for img in $images; do docker rmi $img || true; done

#  docker pull --quiet golang:1.18-alpine
#  images=$(docker images --filter "dangling=true" --quiet golang)
#  for img in $images; do docker rmi $img || true; done
#} &> /dev/null
for base in $(awk '/^FROM/{print $2}' ${_path}/Dockerfile); do
    docker pull --quiet $base
    bn=$(echo $base | awk -F ":" '{print $1}')
    if [[ -z "$bn" ]]; then continue; fi
    docker images --filter "dangling=true" --quiet "$bn" | xargs -i docker rmi {}
done &> /dev/null

echo ">>> build image: $image:$tag..."

docker build --no-cache --file ${_path}/Dockerfile --tag $image:$tag \
  --build-arg=BuildTime="$BuildTime"      \
  --build-arg=BuildBranch="$BuildBranch"  \
  --build-arg=CommitHash="$CommitHash"    \
  --build-arg=CommitTime="$CommitTime" ./

docker image prune --force --filter label=stage=hello_builder &> /dev/null

#### push image
echo ">>> push image: $image:$tag..."
docker push $image:$tag

end_s=$(date +%s)
echo -e "$start_t\tbuild\t$tag\t$((end_s - start_s))" >> $log_file

images=$(docker images --filter "dangling=true" --quiet $image)
for img in $images; do docker rmi $img || true; done &> /dev/null
