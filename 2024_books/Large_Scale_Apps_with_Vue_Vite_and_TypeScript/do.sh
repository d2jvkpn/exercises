#!/bin/bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0)

exit
npm init vite@latest
# interactive

# npm install -D esbuild@0.24.0

cd vite-vue-ts

npm install
npm run dev

exit
npm install vite-tsconfig-paths

npm install --save-dev @types/node

exit
npm update
npm fund
