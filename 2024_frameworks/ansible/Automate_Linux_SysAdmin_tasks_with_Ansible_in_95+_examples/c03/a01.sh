#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

ansible-playbook -v a01.yaml

ansible-playbook -vv a01.yaml # debug verbosity
