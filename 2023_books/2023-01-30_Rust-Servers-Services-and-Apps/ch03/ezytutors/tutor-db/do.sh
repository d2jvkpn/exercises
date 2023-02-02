#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
export PROJECT_ROOT=.
cargo run

####
cargo add actix-web@4 actix-rt@2 serde_json@1 dotenv@0.15
cargo add chrono --features=serde
cargo add serde@1 --features=derive
cargo add sqlx@0.6 --features=runtime-actix-rustls,macros,postgres,uuid,chrono,migrate,offline
