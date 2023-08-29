#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
cargo new tcpserver
cargo new tcpclient
cargo new httpserver
cargo new --lib http

cat > Cargo.toml <<EOF
[workspace]
members = [
  "tcpserver", "tcpclient", "http", "httpserver",
]
EOF

cat > .rustfmt.toml <<EOF
use_small_heuristics = "Max"
EOF

####
cargo run --bin tcpserver
cargo run --bin tcpclient

cargo run --bin httpserver


####
curl http://localhost:3000/
curl http://localhost:3000/health
curl http://localhost:3000/api/shipping/orders
curl http://localhost:3000/invalid-path
