#!/bin/bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0)


npm init vite@latest
# interactive

npm i -D esbuild@0.24.0

cd vite-project
npm install
npm run format
npm run dev
