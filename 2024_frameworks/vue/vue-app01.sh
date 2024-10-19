#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

#### 1.
npm install -g @vue/cli

# npm init vite@latest
# my-vue-project
# Vue
# TypeScript


#### 2.
which vue
command -v vue

vue create -d vue-app01

cd vue-app01

npm install

npm update

npm run serve -- --port=3000
