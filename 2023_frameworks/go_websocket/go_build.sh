#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


buildTime=$(date +'%FT%T%:z')
gitBranch="$(git rev-parse --abbrev-ref HEAD)" # current branch
gitCommit=$(git rev-parse --verify HEAD) # git log --pretty=format:'%h' -n 1
gitTime=$(git log -1 --format="%at" | xargs -I{} date -d @{} +%FT%T%:z)

uncommitted=$(git status --short)
unpushed=$(git diff origin/$gitBranch..HEAD --name-status)

if [[ $(printenv APP_GitForce) != "true" ]]; then
    test -z "$uncommitted" || { echo "You have uncommitted changes!"; exit 1; }
    test -z "$unpushed" || { echo "You have unpushed commits!"; exit 1; }
fi

ldflags="\
  -X main.buildTime=${buildTime} \
  -X main.gitBranch=$gitBranch   \
  -X main.gitCommit=$gitCommit   \
  -X main.gitTime=$gitTime"

go build -ldflags="$ldflags" -o target/hello main.go
echo "saved target/hello"
# GOOS=windows GOARCH=amd64 go build -ldflags="$ldflags" -o target/hello.exe main.go

# p=target/hello_$b1.$(
#   echo $BuildTime |
#   awk -F "T" '{gsub(":", "-", $0); print $1"T"substr($2, 1, 8)}'
# ).zip

# mkdir -p $(dirname $p)
# zip -r -j $p target/hello
# echo "save $p"
