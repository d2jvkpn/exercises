#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


npm init vite@latest vue-app -- --template=vue

cd vue-app
npm install
