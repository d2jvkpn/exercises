#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

rustc --version
cargo --version

cargo install cargo-tarpaulin cargo-audit cargo-edit cargo-expand cargo-udeps
rustup component add clippy rustfm

# cargo new zero2prod && cd zero2prod
mkdir -p zero2prod && cd zero2prod
cargo init

mkdir -p tests
cargo add actix-web actix_rt tokio config chrono
cargo add sqlx --features "runtime-actix-rustls macros postgres uuid chrono migrate offline"
cargo add serde --features=derive
cargo add uuid --features "v4 serde"
cargo add --dev reqwest
cat Cargo.toml

echo 'use_small_heuristics = "Max"' >> .rustfmt.toml

# git add .
# git commit -am "Project skeleton"
# git remote add origin git@github.com:YourGitHubNickName/zero2prod.git
# git push -u origin main

cargo clippy
cargo clippy -- -D warnings

caego fmt
cargo fmt -- --check

cargo audit
cargo expand

cargo run
curl localhost:8000

#### do sqlx-cli migrations
