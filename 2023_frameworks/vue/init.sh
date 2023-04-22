#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


npm init vue@3
# interactive

npm install
npm run format
npm run dev
