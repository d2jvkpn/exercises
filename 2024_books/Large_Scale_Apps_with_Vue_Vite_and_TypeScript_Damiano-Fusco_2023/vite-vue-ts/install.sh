#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


app="$1"
# npm init vite@latest # interactive
npm init vite@latest $app -- --template=vue-ts

cd $app

npm install
npm run dev

exit
npm i -D axios --save

npm update
npm fund

exit
npm install --global yarn
yarn --version

exit
npm install -D esbuild@0.24.0

npm install vite-tsconfig-paths

#npm install --save-dev @types/node
