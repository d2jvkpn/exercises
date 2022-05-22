#! /usr/bin/env bash
set -eu -o pipefail

_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

find -type f -name  Cargo.toml |
  xargs -i dirname {} |
  xargs -I {} cp .rustfmt.toml {}/


git remote get-url --all
# output: https://github.com/d2jvkpn/Mastering-RUST-Second-Edition

git remote set-url origin ssh://git@github.com/d2jvkpn/Mastering-RUST-Second-Edition.git

# git config user.name "d2jvkpn"
# config user.email "xxxx@yyyy.com"
