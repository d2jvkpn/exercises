#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

pip3 config set global.index-url 'https://pypi.douban.com/simple/'
pip3 config set install.trusted-host 'pypi.douban/simple'

pip3 config list

pip3 install pandas numpy polars
