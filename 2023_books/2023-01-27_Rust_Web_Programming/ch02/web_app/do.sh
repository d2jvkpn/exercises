#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
cargo new web_app && cd web_app
cargo add actix-web
mkdir -p tests

cat > .rustfmt.tom <<EOF
use_small_heuristics = "Max"
EOF

cargo add tokio@1 --features=full
