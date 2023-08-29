#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


mkdir -p target

export GIT_COMMIT=$(git rev-list -1 HEAD) && echo $GIT_COMMIT

go build -ldflags "-X main.GitCommit=$GIT_COMMIT" -o target/main && \
  target/main -version
