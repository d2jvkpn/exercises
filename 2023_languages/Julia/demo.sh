#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

./xlsx_to_sql.jl data/browsenode.xlsx data/browsenode.sql browse_node
