#!/bin/bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0)

exit
npm init vite@latest
# interactive

npm i -D esbuild@0.24.0

cd vite-vue-01

npm install
npm run format
npm run dev

exit
npm install vite-tsconfig-paths

npm update
npm fund
