#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

# echo "Hello, world!"

npm init vite@latest
# my-vue-project
# Vue
# TypeScript

cd my-vue-project
npm install
npm run dev
