#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo generate --git https://github.com/rustwasm/wasm-pack-template --name hello-wasm
# hello-wasm

cargo new hello-actix

cd hello-actix

cargo add actix-web actix-rt
cargo add serde --features=serde_derive
# cargo add tera
cargo add askama --features=with-actix-web,serde,serde-json,serde-yaml,markdown
cargo add askama_actix --features=serde-json,serde-yaml,urlencode,markdown
cargo add tokio --features=full
cargo add hyper
