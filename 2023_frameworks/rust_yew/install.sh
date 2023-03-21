#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

rustup target add wasm32-unknown-unknown

cargo install cargo-watch

cargo install --locked trunk
command trunk

cargo install -f wasm-bindgen-cli
command wasm-bindgen


# cargo generate --git https://github.com/yewstack/yew-trunk-minimal-template

cargo new demo-01
cd demo-01

cargo run
cargo add yew@0.20 --features=csr
# features: hydration,ssr

trunk serve


exit

references:

https://yew.rs/docs/getting-started/build-a-sample-app

https://rustwasm.github.io/wasm-bindgen/reference/cli.html
