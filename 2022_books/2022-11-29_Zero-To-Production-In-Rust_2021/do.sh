#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


rustc --version
cargo --version

cargo install cargo-tarpaulin cargo-audit cargo-edit cargo-expand
rustup component add clippy rustfm


cargo new a01
cd a01
mkdir -p tests
cargo add actix-web actix_rt tokio
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

