#!/bin/bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0)

exit
npm init vite@latest
# interactive

cd vite-vue-ts

npm install --save-dev @types/node
npm install
npm run dev

exit
npm update
npm fund

exit
npm install --global yarn
yarn --version

exit
npm install -D esbuild@0.24.0
npm install vite-tsconfig-paths
