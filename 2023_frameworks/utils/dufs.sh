#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo install dufs

# --auth {path}@{username}@{password}

mkdir -p alice bob
echo "Hello" > alice/hello.txt
echo "Hello" > bob/hello.txt

dufs --bind 127.0.0.1 --port 3000 \
  --allow-upload --allow-search \
  --auth /@admin:admin \
  --auth /alice@alice:alice \
  --auth /bob@bob:bob
