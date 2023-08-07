#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

export JULIA_PKG_SERVER=https://mirrors.ustc.edu.cn/julia

