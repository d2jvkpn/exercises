#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

# echo "Hello, world!"

npm install -g typesctipt
tsc --verison


mkdir -p p01 && cd p01

tsc --init
cat tsconfig.json

echo 'console.log("Nothing is worth more than laughter.");' > index.ts
tsc index.ts

node index.js
